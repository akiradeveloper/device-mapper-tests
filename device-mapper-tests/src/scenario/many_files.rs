use crate::*;
use cmd_lib::*;

pub struct ManyFiles {
    pub n: usize,
}
impl ManyFiles {
    pub fn run_on(&self, root: &str) {
        let n = self.n;
        for i in 0..n {
            run_cmd!(touch $root/$i).unwrap();
            run_cmd!(echo $i > $root/$i).unwrap();
        }
        kernel::drop_caches();
        for i in 0..n {
            let x = format!("{}", i);
            let v = run_fun!(cat $root/$i).unwrap();
            assert_eq!(v, x);
            run_cmd!(rm $root/$i).unwrap();
        }
    }
}
