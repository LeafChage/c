use super::parser::Node;
use std::io::Result;

fn equal(left: Node, right: Node) -> Result<()>{
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

fn unequal(left: Node, right: Node) -> Result<()>{
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

fn less(left: Node, right: Node) -> Result<()>{
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
fn less_equal(left: Node, right: Node) -> Result<()>{
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

fn plus(left: Node, right: Node) -> Result<()>{
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  add rax, rdi");
    println!("  push rax");
    Ok(())
}

fn minus(left: Node, right: Node) -> Result<()>{
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  sub rax, rdi");
    println!("  push rax");
    Ok(())
}

fn multiple(left: Node, right: Node) -> Result<()>{
    node(left)?;
    node(right)?;
    println!("  pop rdi");
    println!("  pop rax");
    println!("  imul rax, rdi");
    println!("  push rax");
    Ok(())
}

fn devide(left: Node, right: Node) -> Result<()>{
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
        // Node::Unary(n) => {
        //     println!("  push {}", n);
        // }
        Node::Equal(left, right) => equal(*left, *right)?,
        Node::UnEqual(left, right) => unequal(*left, *right)?,
        Node::Less(left, right) => less(*left, *right)?,
        Node::LessEqual(left, right) => less_equal(*left, *right)?,
        Node::Plus(left, right) => plus(*left, *right)?,
        Node::Minus(left, right) => minus(*left, *right)?,
        Node::Multiple(left, right) => multiple(*left, *right)?,
        Node::Devide(left, right) => devide(*left, *right)?,
        _ => unimplemented!(),
    }
    Ok(())
}

pub fn gen(n: Node) -> Result<()> {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    node(n)?;
    println!("  pop rax");
    println!("  ret");
    Ok(())
}

