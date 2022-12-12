use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::astar;
use std::collections::HashMap;

fn add_edge(x1: usize, y1: usize, x2: usize, y2: usize, node_coords: &HashMap::<(usize,usize), NodeIndex>, g: &mut Graph::<char,u32>) {
  let index1 = *node_coords.get(&(x1,y1)).expect("no_node1");
  let index2 = *node_coords.get(&(x2,y2)).expect("no_node2");
  if(g[index1].to_ascii_lowercase() as u32) >= (g[index2].to_ascii_lowercase() as u32 - 1) {
    //println!("{x1} {y1} {:?}; {x2} {y2} {:?};", g[index1], g[index2]);
    g.add_edge(index1, index2, 1);
  }
}

pub fn run(stdout: &mut dyn std::io::Write, content : &str) {
  let lines: Vec<&str> = content.split("\n").collect();

  let mut start: Option<NodeIndex> = None;
  let mut target: Option<NodeIndex> = None;
  let mut g = Graph::<char,u32>::new();
  let mut node_coords = HashMap::<(usize,usize), NodeIndex>::new();

  for (y,line) in lines.iter().enumerate() {
    for (x,c) in line.chars().enumerate() {
      let h = match c {
        'S' => {'a'},
        'E' => {'z'},
        h => { h },
      };
      let cur = g.add_node(h);
      if c == 'S' { start = Some(cur); }
      if c == 'E' { target = Some(cur); }
      node_coords.insert((x,y), cur);
    }
  }

  for (y,line) in lines.iter().enumerate() {
    for (x,_c) in line.chars().enumerate() {
      if y > 0             { add_edge(x,y,x,y-1, &node_coords, &mut g); }
      if y < lines.len()-1 { add_edge(x,y,x,y+1, &node_coords, &mut g); }
      if x > 0             { add_edge(x,y,x-1,y, &node_coords, &mut g); }
      if x < line.len()-1  { add_edge(x,y,x+1,y, &node_coords, &mut g); }
    }
  }

  let path = astar(&g, start.expect("s"), |n| n == target.expect("t"), |e| *e.weight(), |_| 0);  
  let mut min_steps = path.expect("no path!").0;
  writeln!(stdout, "{min_steps}").ok();

  for index in g.node_indices().filter(|i| g[*i] == 'a') {
    let new_path = astar(&g, index, |n| n == target.expect("t"), |e| *e.weight(), |_| 0);
    let steps = match new_path {
      Some(path) => path.0,
      _ => min_steps
    };
    if(steps < min_steps) { min_steps = steps; }
  }
  writeln!(stdout, "{min_steps}").ok();
}