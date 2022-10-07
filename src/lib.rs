use std::{io, collections::HashMap};

/// # 解释器
/// memory: 寄存器
/// 
/// mem_pointer: 寄存器指针
/// 
/// tape: 纸带内容
/// 
/// pointer: 纸带指针
/// 
/// bracket_cache: 括号配对
pub struct Compiler {
  memory: Vec<u8>,
  tape: Vec<char>,
  pointer: Box<usize>,
  mem_pointer: Box<usize>,
  bracket_cache: HashMap<usize, usize>
}

impl Compiler {
  pub fn new(contents: &String) -> Compiler {
    let memory: Vec<u8> = vec![0;1000];
    let mem_pointer: Box<usize> = Box::new(0);
    let tape: Vec<char> = contents.chars().collect();
    let pointer: Box<usize> = Box::new(0);
    let bracket_cache: HashMap<usize, usize> = HashMap::new();

    Compiler {
      memory,
      tape,
      pointer,
      mem_pointer,
      bracket_cache,
    }
  }
  /// 解释器
  pub fn interpret(&mut self) {
    self.bracket();
    loop {
      self.bitwise();
      if *self.pointer >= self.tape.len() {
        break;
      }
    }
  }
  /// 位处理解释器
  fn bitwise(&mut self) {
    match self.tape[*self.pointer] {
      '>' => *self.mem_pointer += 1,
      '<' => *self.mem_pointer -= 1,
      '+' => self.memory[*self.mem_pointer] = self.memory[*self.mem_pointer].wrapping_add(1),
      '-' => self.memory[*self.mem_pointer] = self.memory[*self.mem_pointer].wrapping_sub(1),
      '.' => {
        print!("{}", self.memory[*self.mem_pointer] as char);
      },
      ',' => {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        self.memory[*self.mem_pointer] = buf.chars().next().unwrap() as u8;
      },
      '[' => if self.memory[*self.mem_pointer] == 0 {
        *self.pointer = *self.bracket_cache.get(&self.pointer).unwrap();
      },
      ']' => if self.memory[*self.mem_pointer] != 0 {
        *self.pointer = *self.bracket_cache.get(&self.pointer).unwrap();
      },
      _ => ()
    }
    *self.pointer += 1;
  }
  /// 中括号处理器
  fn bracket(&mut self) {
    let mut stack = Vec::new();

    for index in 0..self.tape.len() {
      match self.tape[index] {
        '[' => stack.push(index),
        ']' => {
          let left = match stack.pop() {
            Some(i) => i,
            _ => panic!("No matched brackets at: {}", index),
          };
          self.bracket_cache.insert(left, index);
          self.bracket_cache.insert(index, left);
        },
        _ => ()
      }
    }
  }
}