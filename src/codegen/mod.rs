use super::parser::Node;
use std::io::Result;

fn definition_variable(_id: String, offset: usize) -> Result<()> {
    println!("  mov rax, rbp");
    println!("  sub rax, {}", offset);
    println!("  push rax");
    Ok(())
}

fn local_val(id: String, offset: usize) -> Result<()> {
    definition_variable(id, offset)?;
    println!("  pop rax");
    println!("  mov rax, [rax]");
    println!("  push rax");
    Ok(())
}

fn assign(left: Node, right: Node) -> Result<()> {
    if let Node::LocalVariable(id, offset) = left {
        definition_variable(id, offset)?;
    } else {
        unreachable!();
    }
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  mov [rax], rdi");
    println!("  push rdi");
    Ok(())
}

fn rtn(n: Node) -> Result<()> {
    node(n)?;
    println!("  pop rax");
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
    Ok(())
}

fn equal(left: Node, right: Node) -> Result<()> {
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  cmp rax, rdi");
    // sete 命令
    // cmp命令での結果を指定したレジスタにコピーする
    println!("  sete al");
    // sete命令が8bitレジスタにしか扱えないので
    // movzb命令で上位56bitをゼロクリアして持ってくる
    println!("  movzb rax, al");
    println!("  push rax");

    Ok(())
}

fn unequal(left: Node, right: Node) -> Result<()> {
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  cmp rax, rdi");
    println!("  setne al");
    println!("  movzb rax, al");
    println!("  push rax");
    Ok(())
}

fn less(left: Node, right: Node) -> Result<()> {
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  cmp rax, rdi");
    println!("  setl al");
    println!("  movzb rax, al");
    println!("  push rax");
    Ok(())
}
fn less_equal(left: Node, right: Node) -> Result<()> {
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  cmp rax, rdi");
    println!("  setle al");
    println!("  movzb rax, al");
    println!("  push rax");
    Ok(())
}

fn plus(left: Node, right: Node) -> Result<()> {
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  add rax, rdi");
    println!("  push rax");
    Ok(())
}

fn minus(left: Node, right: Node) -> Result<()> {
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  sub rax, rdi");
    println!("  push rax");
    Ok(())
}

fn multiple(left: Node, right: Node) -> Result<()> {
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  imul rax, rdi");
    println!("  push rax");
    Ok(())
}

fn devide(left: Node, right: Node) -> Result<()> {
    /*
     * idivは符号あり除算を行う命令です。
     * x86-64のidivが素直な仕様になっていれば、
     * 上のコードでは本来idiv rax, rdiのように書きたかったところですが、
     * そのような2つのレジスタをとる除算命令はx86-64には存在しません。
     * その代わりに、
     * idivは暗黙のうちにRDXとRAXを取って、
     * それを合わせたものを128ビット整数とみなして、
     * それを引数のレジスタの64ビットの値で割り、
     * 商をRAXに、余りをRDXにセットする、という仕様になっています。
     * cqo命令を使うと、RAXに入っている64ビットの値を128ビットに伸ばしてRDXとRAXにセットすることができるので、
     * 上記のコードではidivを呼ぶ前にcqoを呼んでいます。
     */
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  cqo");
    println!("  idiv rdi");
    println!("  push rax");
    Ok(())
}

fn node(n: Node) -> Result<()> {
    match n {
        Node::Number(n) => {
            println!("  push {}", n);
        }
        Node::Equal(left, right) => equal(*left, *right)?,
        Node::UnEqual(left, right) => unequal(*left, *right)?,
        Node::Less(left, right) => less(*left, *right)?,
        Node::LessEqual(left, right) => less_equal(*left, *right)?,
        Node::Plus(left, right) => plus(*left, *right)?,
        Node::Minus(left, right) => minus(*left, *right)?,
        Node::Multiple(left, right) => multiple(*left, *right)?,
        Node::Devide(left, right) => devide(*left, *right)?,
        Node::Assign(left, right) => assign(*left, *right)?,
        Node::LocalVariable(id, offset) => local_val(id, offset)?,
        Node::Return(n) => rtn(*n)?,
        Node::If(..) => unimplemented!(),
        Node::While(..) => unimplemented!(),
        Node::For(..) => unimplemented!(),
    }
    Ok(())
}

fn prologue() -> Result<()> {
    // allocate for 26 variable argument
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");
    Ok(())
}

pub fn gen(nodes: Vec<Node>) -> Result<()> {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    prologue()?;
    for n in nodes {
        node(n)?;
        println!("  pop rax");
    }
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
    Ok(())
}
