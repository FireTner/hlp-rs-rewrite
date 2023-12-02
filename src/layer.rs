use crate::vec::i8x16;

#[derive(Copy, Clone)]
pub struct Layer {
  pub layer: i8x16,
  pub layerconf: u16,
}

// Possible configurations of single layer
#[derive(FromPrimitive)]
pub enum LayerConf {
  ComCom = 0,
  SubCom,
  ComSub,
  SubSub,
  SubCom2,
  Size,
}

// Amount of states comparators can possibly be in
pub const CONFCOUNT: u16 = LayerConf::Size as u16 * 256;

// Comparator in compare mode
// Takes back and side as parameters
// Returns i8x16 of comparator results
//
// Calculates results of comparator in compare mode of every element
fn compare(back: &i8x16, side: &i8x16) -> i8x16 {
  back.cmplt(&side).andnot(back)
}

// Comparator in compare mode
// Takes back and side as parameters
// Returns i8x16 of subtraction results
//
// Calculates results of comparator in subtraction mode of every element
fn subtract(back: &i8x16, side: &i8x16) -> i8x16 {
  i8x16::max(&(*back - *side),& i8x16::zero())
}

impl Layer {
  // Generates a layer
  // Takes input and configuration as parameters
  // Returns a layer generated from input and configuration
  //
  // Calculates every comparator configuration of single layer
  pub fn generate(input: i8x16, configuration: u16) -> Self {
    let back = i8x16::from_imm(( configuration       & 0xF) as i8);
    let side = i8x16::from_imm(((configuration >> 4) & 0xF) as i8);

  // Takes 3 bits starting from 8th bit as configuration of comparators and casts it to enum
    let layer_conf = num::FromPrimitive::from_u16((configuration >> 8) & 7);

    let layer = match layer_conf {
      None => i8x16::zero(),
      Some(x) => match x {
        LayerConf::ComCom  => i8x16::max(&compare (&back, &input), &compare (&input, &side)),
        LayerConf::SubCom  => i8x16::max(&subtract(&back, &input), &compare (&input, &side)),
        LayerConf::ComSub  => i8x16::max(&compare (&back, &input), &subtract(&input, &side)),
        LayerConf::SubSub  => i8x16::max(&subtract(&back, &input), &subtract(&input, &side)),
        LayerConf::SubCom2 => i8x16::max(&subtract(&back, &input), &compare (&side, &input)),
        _ => i8x16::zero(),
      },
    };

    Layer {
      layerconf: configuration,
      layer: layer,
    }
  }

  // Prints the configuration of a layer
  // Takes Layer as a parameter
  //
  // Prints configuration as a string
  // Format: SV,_SV;_ where S is symbol and V is 1 character value
  pub fn print(self) {
    let back =  self.layerconf       & 0xF;
    let side = (self.layerconf >> 4) & 0xF;

  // Takes 3 bits starting from 8th bit as configuration of comparators and casts it to enum
    let layer_conf = num::FromPrimitive::from_u16((self.layerconf >> 8) & 7);

    match layer_conf {
      None => eprint!(" invalid configuration "),
      Some(x) => match x {
        LayerConf::ComCom  => print!(" {:1X},  {:1X}; ", back, side),
        LayerConf::SubCom  => print!("*{:1X},  {:1X}; ", back, side),
        LayerConf::ComSub  => print!(" {:1X}, *{:1X}; ", back, side),
        LayerConf::SubSub  => print!("*{:1X}, *{:1X}; ", back, side),
        LayerConf::SubCom2 => print!(">{:1X}, <{:1X}; ", back, side),
        _ => eprint!(" invalid configuration "),
      },
    };
  }

  // Returns an empty layer
  pub const fn empty() -> Layer {
    Layer { layer: i8x16::zero(), layerconf: 0 }
  }
}

// Implements equality check
// this only checks equality of layer member
impl PartialEq for Layer {
  fn eq(&self, other: &Self) -> bool {
    self.layer == other.layer
  }

  fn ne(&self, other: &Self) -> bool {
    self.layer != other.layer
  }
}