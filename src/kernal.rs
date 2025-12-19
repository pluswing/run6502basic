use crate::cpu::{AddressingMode, CPU};
use std::io::{self, Write};

#[derive(Default)]
pub struct Kernal {
  line_input: String,
  line_index: usize,
}
impl Kernal {
  pub fn new() -> Self {
    Self {
      line_input: "".to_string(),
      line_index: 0,
    }
  }

  pub fn handle(&mut self, cpu: &mut CPU) -> bool {
    match cpu.program_counter {
      0xFFE7 => {
        // CLALL
        // 初期化をするルーチンらしい
        // 何もしなくてよさそう。
        cpu.rts(&AddressingMode::Absolute);
      }
      0xFFD2 => {
        // CHROUT, MONCOUT
        // -> 画面出力系
        // Aレジスタの値を取る
        // asciiに変換する
        // 画面にprintする
        // かえる
        let char_value = cpu.register_a as char;
        print!("{}", char_value);
        cpu.rts(&AddressingMode::Absolute);
      }
      0xFFCF => {
        // CHRIN
        // キーボードからの入力は特別な方法で処理されます。
        // まず、カーソルが点灯し、
        // キーボードでキャリッジリターンが入力されるまで点滅します。
        // 行内のすべての文字（最大88文字）はBASICの入力バッファに格納されます。
        // これらの文字は、このルーチンを文字ごとに1回呼び出すことで、一度に1文字ずつ取得できます。
        // キャリッジリターンが取得されると、行全体の処理が完了します。
        // 次にこのルーチンが呼び出されると、カーソルが点滅するというプロセスが再開されます。
        if self.line_input.len() == 0 {
          // 1行入力させる
          print!("> ");
          io::stdout().flush();
          std::io::stdin()
            .read_line(&mut self.line_input)
            .unwrap();
          self.line_input = self.line_input.replace("\n", "\r");
          self.line_index = 0;
          return true;
        } else {
          // 1文字づつ返す
          let c = self.line_input.chars().nth(self.line_index).unwrap();
          cpu.register_a = c as u8;
          self.line_index += 1;
          if cpu.register_a == 13 {
            // CRだったら終わり
            self.line_input = "".to_string();
          }
          cpu.rts(&AddressingMode::Absolute);
        }
      }
      0xFFE1 => {
        // STOP
        // self.register_a = 0;
        cpu.rts(&AddressingMode::Absolute);
      }
      0xFFCC => {
        // CLRCHN
        // self.register_a = 0;
        cpu.rts(&AddressingMode::Absolute);
      }
      _ => {}
    }
    false
  }

}

