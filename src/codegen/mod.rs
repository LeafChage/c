use super::parser::Node;
use std::io::Result;

pub struct Codegen {
    block_index: usize,
}

impl Codegen {
    pub fn new() -> Self {
        Self { block_index: 0 }
    }
    fn if_n(&mut self, condition: Node, then: Node, else_body: Option<Node>) -> Result<()> {
        let block_index = self.block_index;
        self.block_index += 1;

        self.node(condition)?;
        println!("  pop rax");
        println!("  cmp rax, 0");

        if let Some(else_body) = else_body {
            println!("  je .Lelse{}", block_index);
            self.node(then)?;
            println!("  jmp .Lend{}", block_index);
            println!(".Lelse{}:", block_index);
            self.node(else_body)?;
        } else {
            println!("  je .Lend{}", block_index);
            self.node(then)?;
        }

        println!(".Lend{}:", block_index);
        Ok(())
    }

    fn whlie_n(&mut self, condition: Node, body: Node) -> Result<()> {
        let block_index = self.block_index;
        self.block_index += 1;

        println!(".Lbegin{}:", block_index);
        self.node(condition)?;

        println!("  pop rax");
        println!("  cmp rax, 0");
        println!("  je .Lend{}", block_index);
        self.node(body)?;
        println!("  jmp .Lbegin{}", block_index);
        println!(".Lend{}:", block_index);

        Ok(())
    }

    fn for_n(
        &mut self,
        condition1: Option<Node>,
        condition2: Option<Node>,
        condition3: Option<Node>,
        body: Node,
    ) -> Result<()> {
        let block_index = self.block_index;
        self.block_index += 1;

        if let Some(cdn) = condition1 {
            self.node(cdn)?;
        }
        println!(".Lbegin{}:", block_index);
        if let Some(cdn) = condition2 {
            self.node(cdn)?;
        }

        println!("  pop rax");
        println!("  cmp rax, 0");
        println!("  je .Lend{}", block_index);
        if let Some(cdn) = condition3 {
            self.node(cdn)?;
        }
        self.node(body)?;
        println!("  jmp .Lbegin{}", block_index);
        println!(".Lend{}:", block_index);

        Ok(())
    }

    fn definition_variable(&mut self, _id: String, offset: usize) -> Result<()> {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", offset);
        println!("  push rax");
        Ok(())
    }

    fn local_val(&mut self, id: String, offset: usize) -> Result<()> {
        self.definition_variable(id, offset)?;
        println!("  pop rax");
        println!("  mov rax, [rax]");
        println!("  push rax");
        Ok(())
    }

    fn assign(&mut self, left: Node, right: Node) -> Result<()> {
        if let Node::LocalVariable(id, offset) = left {
            self.definition_variable(id, offset)?;
        } else {
            unreachable!();
        }
        self.node(right)?;
        println!("  pop rdi");
        println!("  pop rax");
        println!("  mov [rax], rdi");
        println!("  push rdi");
        Ok(())
    }

    fn return_n(&mut self, n: Node) -> Result<()> {
        self.node(n)?;
        println!("  pop rax");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
        Ok(())
    }

    fn equal(&mut self, left: Node, right: Node) -> Result<()> {
        self.node(left)?;
        self.node(right)?;
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

    fn unequal(&mut self, left: Node, right: Node) -> Result<()> {
        self.node(left)?;
        self.node(right)?;
        println!("  pop rdi");
        println!("  pop rax");
        println!("  cmp rax, rdi");
        println!("  setne al");
        println!("  movzb rax, al");
        println!("  push rax");
        Ok(())
    }

    fn less(&mut self, left: Node, right: Node) -> Result<()> {
        self.node(left)?;
        self.node(right)?;
        println!("  pop rdi");
        println!("  pop rax");
        println!("  cmp rax, rdi");
        println!("  setl al");
        println!("  movzb rax, al");
        println!("  push rax");
        Ok(())
    }
    fn less_equal(&mut self, left: Node, right: Node) -> Result<()> {
        self.node(left)?;
        self.node(right)?;
        println!("  pop rdi");
        println!("  pop rax");
        println!("  cmp rax, rdi");
        println!("  setle al");
        println!("  movzb rax, al");
        println!("  push rax");
        Ok(())
    }

    fn plus(&mut self, left: Node, right: Node) -> Result<()> {
        self.node(left)?;
        self.node(right)?;
        println!("  pop rdi");
        println!("  pop rax");
        println!("  add rax, rdi");
        println!("  push rax");
        Ok(())
    }

    fn minus(&mut self, left: Node, right: Node) -> Result<()> {
        self.node(left)?;
        self.node(right)?;
        println!("  pop rdi");
        println!("  pop rax");
        println!("  sub rax, rdi");
        println!("  push rax");
        Ok(())
    }

    fn multiple(&mut self, left: Node, right: Node) -> Result<()> {
        self.node(left)?;
        self.node(right)?;
        println!("  pop rdi");
        println!("  pop rax");
        println!("  imul rax, rdi");
        println!("  push rax");
        Ok(())
    }

    fn devide(&mut self, left: Node, right: Node) -> Result<()> {
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
        self.node(left)?;
        self.node(right)?;
        println!("  pop rdi");
        println!("  pop rax");
        println!("  cqo");
        println!("  idiv rdi");
        println!("  push rax");
        Ok(())
    }

    fn node(&mut self, n: Node) -> Result<()> {
        match n {
            Node::Number(n) => {
                println!("  push {}", n);
            }
            Node::Equal(left, right) => self.equal(*left, *right)?,
            Node::UnEqual(left, right) => self.unequal(*left, *right)?,
            Node::Less(left, right) => self.less(*left, *right)?,
            Node::LessEqual(left, right) => self.less_equal(*left, *right)?,
            Node::Plus(left, right) => self.plus(*left, *right)?,
            Node::Minus(left, right) => self.minus(*left, *right)?,
            Node::Multiple(left, right) => self.multiple(*left, *right)?,
            Node::Devide(left, right) => self.devide(*left, *right)?,
            Node::Assign(left, right) => self.assign(*left, *right)?,
            Node::LocalVariable(id, offset) => self.local_val(id, offset)?,
            Node::Return(n) => self.return_n(*n)?,
            Node::If(condition, then, else_body) => {
                self.if_n(*condition, *then, else_body.map(|e| *e))?
            }
            Node::While(condition, body) => self.whlie_n(*condition, *body)?,
            Node::For(condition1, condition2, condition3, body) => self.for_n(
                condition1.map(|c| *c),
                condition2.map(|c| *c),
                condition3.map(|c| *c),
                *body,
            )?,
        }
        Ok(())
    }

    fn prologue(&mut self) -> Result<()> {
        // allocate for 26 variable argument
        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, 208");
        Ok(())
    }

    pub fn gen(&mut self, nodes: Vec<Node>) -> Result<()> {
        println!(".intel_syntax noprefix");
        println!(".globl main");
        println!("main:");
        self.prologue()?;
        for n in nodes {
            self.node(n)?;
            println!("  pop rax");
        }
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
        Ok(())
    }
}

pub fn codegen(nodes: Vec<Node>) -> Result<()> {
    let mut c = Codegen::new();
    c.gen(nodes)
}
