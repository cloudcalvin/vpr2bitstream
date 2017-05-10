use vpr_extra::global::*;
use vpr_extra::types::*;
//use vpr_extra::types::Logic;

pub trait NoDangle {
  //  fn remove_to_dangling_input_ports(model : Model);
  fn connect_dangling_input_ports(&mut self);
  //  fn replace_dont_care(vec: Vec<Logic>) -> Vec<String>;
}

//pub inputs : Vec<String>,
//pub output : String,
////  pub latched : bool,
//pub truth_idxs : Vec<usize>,
//pub truth_table : Vec<bool>

impl NoDangle for Model {
  fn connect_dangling_input_ports(&mut self) {
    for ref mut lb in &mut self.logic {
      let mut inputs = &mut lb.inputs;
      if inputs.len() < *K_LUT as usize {
        info_println!("logic block {:?} has {:?} unconnected input ports: {:?}",&lb.output,*K_LUT-inputs.len() as  u16,&inputs);

        let first: String = inputs.first().unwrap().clone();
        let mut new_tt: Vec<bool> = Vec::from(lb.truth_table.get(0..(2 as u32).pow(inputs.len() as u32) as usize).unwrap());

        while inputs.len() < *K_LUT as usize {
          info_println!("reusing input : {:#?} for unconnected ports",first);
          let half = Vec::from(lb.truth_table.get(0..(2 as u32).pow(inputs.len() as u32) as usize).unwrap());
          vv_blif_println!("half : {:?}",&half);
          new_tt = half.clone();
          new_tt.extend(half);
          vv_blif_println!("new tt : {:?}",&new_tt);
          inputs.insert(0, first.clone());
          lb.truth_table = new_tt.clone();
        }

        blif_println!("logic block ports : {:?}",&inputs);
        blif_println!("new truth_table : {:#?}", &lb.truth_table);

        let mut new_idx_list = vec! {};

        for (idx, val) in new_tt.into_iter().enumerate() {
          if val == true {
            new_idx_list.push(idx);
          }
        }
        blif_println!("new idx list : {:#?}", &new_idx_list);
        lb.truth_idxs = new_idx_list;
      } else {
        info_println!("logic block: {:?} not altered",&lb.output);
      }
    }
  }
  //  /// dont-care replacement
  //  fn replace_dont_care(vec: Vec<String>) -> Vec<String> {
  //    let mut new_lut = Vec::new();
  //    let mut done = 1;
  //    for line in vec {
  //      if let Some(i) = line.rfind("-") {
  //        done = 0;
  //        let mut new_line_0: Vec<char> = line.chars().collect();
  //        let mut new_line_1: Vec<char> = line.chars().collect();
  //        new_line_0[i] = '0' as char;
  //        new_lut.push(new_line_0.into_iter().collect::<String>());
  //        new_line_1[i] = '1' as char;
  //        new_lut.push(new_line_1.into_iter().collect::<String>());
  //      } else {
  //        new_lut.push(line)
  //      }
  //    }
  //    if done == 1 {
  //      new_lut.sort();//just remove dups
  //      new_lut
  //    } else {
  //      replace(new_lut)
  //    }
  //  }
}