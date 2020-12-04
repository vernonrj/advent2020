

/**
 * Gonna try to build a tree for this problem
 */
#[derive(Default, Debug, Clone, PartialEq, Eq)]
 pub struct SumTree {
    children: Vec<SumTree>,
    number: i64,
    sum: i64,
    depth: i64,
 }

 impl SumTree {
    pub fn new(max_depth: i64) -> Self {
        SumTree {
            depth: max_depth,
            ..Default::default()
        }
    }
    pub fn insert(&mut self, value: i64) {
        if self.depth <= 0 {
            // limited by the desired tree depth
            return;
        }

        // add a new node to all children
        for child in self.children.iter_mut() {
            child.insert(value);
        }

        // start another node here with the sum up to now
        let new_entry = SumTree {
            number: value,
            sum: self.sum + value,
            depth: self.depth - 1,
            ..Default::default()
        };
        self.children.push(new_entry);
    }
    pub fn find(&self, sum: i64) -> Option<Vec<i64>> {
        if self.depth == 0 && self.sum == sum {
            // hey look we found it!
            Some(vec![self.number])
        } else if !self.children.is_empty() && self.sum < sum {
            // we're still looking, at we haven't gone over the sum we're looking for yet.
            // Go deeper.
            let mut matches = self.children.iter()
                .flat_map(|t| t.find(sum));
            // if we found solutions, take the first one and add our number to it.
            matches.next().map(|mut v: Vec<i64>| {
                    if self.number != 0 {
                        v.push(self.number);
                    }
                    v
            })
        } else {
            // nothing found in this branch :(
            None
        }
    }
 }


 #[test]
 fn test_tree_insert() {
     let mut t = SumTree::new(2);
     t.insert(5);
     assert_eq!(t, SumTree {
         children: vec![SumTree {
            children: vec![],
            number: 5,
            sum: 5,
            depth: 1,
         }],
         number: 0,
         sum: 0,
         depth: 2,
     });

     t.insert(20);
     assert_eq!(t, SumTree {
        children: vec![SumTree {
            children: vec![SumTree {
                children: vec![],
                number: 20,
                sum: 25,
                depth: 0,
           }],
           number: 5,
           sum: 5,
           depth: 1,
        },
        SumTree {
            children: vec![],
            number: 20,
            sum: 20,
            depth: 1,
        }],
        number: 0,
        sum: 0,
        depth: 2,
    });

    t.insert(10);
    assert_eq!(t, SumTree {
        children: vec![SumTree {
            children: vec![
                SumTree {
                    children: vec![],
                    number: 20,
                    sum: 25,
                    depth: 0,
                },
                SumTree {
                    children: vec![],
                    number: 10,
                    sum: 15,
                    depth: 0,
                }
           ],
           number: 5,
           sum: 5,
           depth: 1,
        },
        SumTree {
            children: vec![
                SumTree {
                    children: vec![],
                    number: 10,
                    sum: 30,
                    depth: 0,
                }
            ],
            number: 20,
            sum: 20,
            depth: 1,
        },
        SumTree {
            children: vec![],
            number: 10,
            sum: 10,
            depth: 1,
        }],
        number: 0,
        sum: 0,
        depth: 2,
    });
 }

 #[test]
 fn test_tree_find_d2() {
     let mut t = SumTree::new(2);
     t.insert(1);
     t.insert(2);
     t.insert(3);
     t.insert(4);
     
     assert_eq!(t.find(6), Some(vec![4, 2]));
     assert_eq!(t.find(5), Some(vec![4, 1]));
     assert!(t.find(10).is_none());
 }

 #[test]
 fn test_tree_find_d3() {
     let mut t = SumTree::new(3);
     t.insert(1);
     t.insert(2);
     t.insert(3);
     t.insert(4);
     
     assert_eq!(t.find(6), Some(vec![3, 2, 1]));
     assert_eq!(t.find(9), Some(vec![4, 3, 2]));
 }