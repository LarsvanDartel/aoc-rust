pub struct UnionFind {
    e: Vec<isize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { e: vec![-1; n] }
    }

    pub fn find(&self, x: usize) -> usize {
        if self.e[x] < 0 {
            x
        } else {
            self.find(self.e[x] as usize)
        }
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.e[a] > self.e[b] {
            (a, b) = (b, a);
        }
        self.e[a] += self.e[b];
        self.e[b] = a as isize;
        true
    }

    pub fn size(&self, x: usize) -> usize {
        -self.e[self.find(x)] as usize
    }
}
