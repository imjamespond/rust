use self::bar::bar;

pub(crate) mod bar; // make pub only in crate

pub fn foo(){
  bar();
}