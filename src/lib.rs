use birch::{Directed, Graph};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Push,
    Pop,
    Replace,
}

/// (Top of stack, Input, Action)
pub type Transition = (Option<String>, String, Action);

#[derive(Debug)]
pub struct DPDA {
    pub accept: HashSet<String>,
    pub graph: Graph<String, Transition, Directed>,
}

impl Default for DPDA {
    fn default() -> Self {
        Self::new()
    }
}

impl DPDA {
    pub fn new() -> Self {
        Self {
            accept: HashSet::new(),
            graph: Graph::new(),
        }
    }

    pub fn accept<I: IntoIterator<Item = impl ToString>>(&mut self, acc: I) -> &mut Self {
        for s in acc {
            self.accept.insert(s.to_string());
        }
        self
    }

    pub fn state<'s, I: IntoIterator<Item = (Option<&'s str>, &'s str, Action, &'s str)>>(
        &mut self,
        id: &str,
        transitions: I,
    ) -> &mut Self {
        fn find(graph: &mut Graph<String, Transition, Directed>, id: &str) -> usize {
            match graph.verts.iter().find(|s| match s {
                Some(s) => s.val == id,
                None => false,
            }) {
                Some(s) => s.as_ref().unwrap().index,
                None => graph.add_vert(id.to_owned()),
            }
        }

        let state = find(&mut self.graph, id);

        for (stack, i, act, next) in transitions {
            let next = find(&mut self.graph, next);
            let stack = stack.map(|s| s.to_owned());
            let i = i.to_string();
            self.graph
                .replace_edge(state, (stack.clone(), i.clone(), act), next, move |e| {
                    e.verts.0 == state && e.weight.0 == stack && e.weight.1 == i
                });
        }

        self
    }

    pub fn runner(&self, acc_empty: bool) -> Runner<'_> {
        Runner {
            stack: Vec::new(),
            machine: self,
            acc_empty,
            current: 0,
        }
    }
}

pub struct Runner<'m> {
    pub machine: &'m DPDA,
    pub stack: Vec<String>,
    pub current: usize,
    pub acc_empty: bool,
}

impl<'m> Runner<'m> {
    pub fn run<S: AsRef<str>, I: IntoIterator<Item = S>>(
        &mut self,
        input: I,
    ) -> Result<(Vec<Option<String>>, bool), ()> {
        Ok((
            {
                let mut res = Vec::new();
                for i in input.into_iter() {
                    res.push(self.next(i.as_ref())?);
                }
                res
            },
            self.check(),
        ))
    }

    pub fn next(&mut self, input: &str) -> Result<Option<String>, ()> {
        use Action::*;
        let vert = self.machine.graph.vert(self.current);

        let edge = {
            let top = if self.stack.is_empty() {
                None
            } else {
                Some(&self.stack[self.stack.len() - 1])
            };
            match vert.edges(&self.machine.graph).find(|e| {
                e.weight.1 == input
                    && (top == None
                        || e.weight.0 == None
                        || e.weight.0.as_ref().unwrap() == top.unwrap())
            }) {
                Some(e) => e,
                None => return Err(()),
            }
        };
        self.current = edge.verts.1;
        match edge.weight.2 {
            Push => {
                self.stack.push(input.into());
                Ok(None)
            }
            Pop => Ok(self.stack.pop()),
            Replace => {
                let res = self.stack.pop();
                self.stack.push(input.into());
                Ok(res)
            }
        }
    }

    pub fn check(&self) -> bool {
        (self.acc_empty && self.stack.is_empty())
            || self
                .machine
                .accept
                .contains(&self.machine.graph.vert(self.current).val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine() {
        use Action::*;
        let mut machine = DPDA::new();
        machine
            .state(
                "start",
                vec![(None, "0", Push, "start"), (Some("0"), "1", Pop, "end")],
            )
            .state(
                "end",
                vec![(None, "0", Pop, "end"), (None, "1", Pop, "end")],
            )
            .accept(&["end"]);
        dbg!(&machine);
        assert!(machine.runner(false).run(&["0", "0", "1", "1"]).unwrap().1);
    }
}
