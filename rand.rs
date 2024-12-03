use rand::prelude::*;
use rand::thread_rng;
use std::collections::BinaryHeap;
use rand::distributions::WeightedIndex;


// 定义节点结构体，用于 A* 算法
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    cost: isize,       // 从起点到当前节点的实际代价
    heuristic: isize,  // 启发式估计值（曼哈顿距离）
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
            .then_with(|| self.heuristic.cmp(&other.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// A* 算法实现，结合随机化选择邻居
fn a_star_random_path(map: & mut Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let width = map[0].len();
    let height = map.len();
    let mut rng = thread_rng();

    let mut open_set = BinaryHeap::new(); // 现在存储的是 Reverse<Node>
    let mut came_from = vec![vec![None; width]; height];
    let mut g_score = vec![vec![isize::MAX; width]; height];
    g_score[start.1][start.0] = 0;

    // 使用 std::cmp::Reverse 包装 Node 以实现最小堆
    open_set.push(std::cmp::Reverse(Node { x: start.0, y: start.1, cost: 0, heuristic: 0 }));

    while let Some(std::cmp::Reverse(current_node)) = open_set.pop() {
        if (current_node.x, current_node.y) == end {
            // 重建路径
            let mut path = Vec::new();
            let mut current = end;
            while let Some(prev) = came_from[current.1][current.0] {
                path.push(current);
                current = prev;
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        let neighbors = [
            (current_node.x as isize - 1, current_node.y as isize),
            (current_node.x as isize + 1, current_node.y as isize),
            (current_node.x as isize, current_node.y as isize - 1),
            (current_node.x as isize, current_node.y as isize + 1),
        ];

        // 遍历邻居节点
        for &(nx, ny) in &neighbors {
            // 检查邻居是否在地图范围内
            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                // 如果邻居不是墙壁
                if map[ny][nx] != '#' {
                    let tentative_g_score: isize = g_score[current_node.y][current_node.x] + 1;

                    // 如果找到了更短的路径到邻居
                    if tentative_g_score < g_score[ny][nx] {
                        came_from[ny][nx] = Some((current_node.x, current_node.y));
                        g_score[ny][nx] = tentative_g_score;

                        // 计算曼哈顿距离作为启发式估计值
                        let h = (end.0 as isize - nx as isize).abs() + (end.1 as isize - ny as isize).abs();

                        // 使用 std::cmp::Reverse 包装 Node 以实现最小堆
                        open_set.push(std::cmp::Reverse(Node { x: nx, y: ny, cost: tentative_g_score, heuristic: h }));
                    }
                }
            }
        }

        // 在扩展完所有邻居后，随机选择一个邻居作为下一步
        if let Some(next) = random_neighbor(&came_from, (current_node.x, current_node.y), &map, &end, &mut rng) {
            // 打通路径上的墙壁
            map[next.1][next.0] = '.';
        }
    }

    None
}

// 随机选择一个邻居作为下一步，但优先选择启发式权重较低的邻居
fn random_neighbor(
    came_from: &Vec<Vec<Option<(usize, usize)>>>,
    current: (usize, usize),
    map: &Vec<Vec<char>>,
    end: &(usize, usize),
    rng: &mut ThreadRng,
) -> Option<(usize, usize)> {
    let neighbors = [
        (current.0 as isize - 1, current.1 as isize),
        (current.0 as isize + 1, current.1 as isize),
        (current.0 as isize, current.1 as isize - 1),
        (current.0 as isize, current.1 as isize + 1),
    ];

    // 过滤掉越界、墙壁和已访问的邻居
    let valid_neighbors: Vec<(usize, usize)> = neighbors
        .iter()
        .filter_map(|&(nx, ny)| {
            let nx = nx as usize;
            let ny = ny as usize;
            if nx < map[0].len() && ny < map.len() && map[ny][nx] != '#' && came_from[ny][nx].is_none() {
                Some((nx, ny))
            } else {
                None
            }
        })
        .collect();

    if valid_neighbors.is_empty() {
        return None;
    }

    // 计算每个邻居的启发式权重（曼哈顿距离）
    let weighted_neighbors: Vec<((usize, usize), isize)> = valid_neighbors
        .iter()
        .map(|&(nx, ny)| {
            let dx = (end.0 as isize - nx as isize).abs();
            let dy = (end.1 as isize - ny as isize).abs();
            let heuristic = dx + dy;
            ((nx, ny), heuristic)
        })
        .collect();

    // 按照启发式权重排序，优先选择离目标点更近的邻居
    let mut sorted_neighbors = weighted_neighbors.clone();
    sorted_neighbors.sort_by_key(|&(_, h)| h);

    // 随机选择一个邻居作为下一步，但优先选择启发式权重较低的邻居
    let weights: Vec<isize> = sorted_neighbors.iter().map(|&(_, h)| (100 - h) as isize).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    Some(sorted_neighbors[dist.sample(rng)].0)
}

/// 优化路径，移除不必要的曲折部分。
fn optimize_path(path: &mut Vec<(usize, usize)>, map: &Vec<Vec<char>>) {
    let mut i = 0;
    while i < path.len() - 2 {
        let (x1, y1) = path[i];
        let (x2, y2) = path[i + 2];

        // 检查 (x1, y1) 和 (x2, y2) 之间是否有墙壁
        let can_skip = match (x2 as isize - x1 as isize, y2 as isize - y1 as isize) {
            (dx, 0) => (0..dx.abs()).all(|j| map[y1][(x1 as isize + dx.signum() * j) as usize] != '#'),
            (0, dy) => (0..dy.abs()).all(|j| map[(y1 as isize + dy.signum() * j) as usize][x1] != '#'),
            _ => false,
        };

        if can_skip {
            path.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

/// 生成随机地图并确保有通路
pub fn generate_random_map() -> Vec<Vec<char>> {
    let mut rng = thread_rng();
    let width = {
        let w = rng.gen_range(3..22);
        if w % 2 == 0 {
            w + 1
        } else {
            w
        }
    }; // 确保宽度为奇数以保证有中心线
    let height = rng.gen_range(3..23); // 确保高度足够大
    let wall_prob = rng.gen_range(0.2..0.8); // 墙壁的概率
    
    let mut map = vec![vec!['#'; width]; height];
    let start = (1, 1);
    let end = ((width + 1) / 2, height - 1);

    // 随机放置空地 (.)
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if rng.gen::<f64>() < wall_prob {
                map[y][x] = '.';
            }
        }
    }


    // 从所有位置中随机选择三个不同的位置来放置怪物
    let all_positions: Vec<(usize, usize)> = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .collect();

    let monster_positions: Vec<(usize, usize)> = all_positions
        .choose_multiple(&mut rng, 3)
        .cloned()
        .filter(|&(x, y)| map[y][x] == '.')
        .take(3)
        .collect();

    // 将怪物放置在选中的位置上
    for &(monster_x, monster_y) in &monster_positions {
        map[monster_y][monster_x] = 'M';
    }

    // 直接将玩家放置在指定的位置
    map[start.1][start.0] = 'P';
    
    // 下一层的入口
    map[end.1][end.0] = 'E';

    // 使用 A* 算法检查从起始点到目标点是否有通路
    let path = a_star_random_path(& mut map, start, end);

    // 如果没有通路，重新生成地图
    if path.is_none() {
        println!("Failed to find a path. Regenerating map...");
        return generate_random_map();
    }

    // 打通路径上的墙壁
    if let Some(mut path) = path {
        optimize_path(&mut path, &map);
        for &(x, y) in &path {
            if !(x == end.0 && y == end.1 || x == start.0 && y == start.1) {
                map[y][x] = '.';
            } else if (x == start.0 && y == start.1) {
                // 确保起始位置保持 'P'
                map[y][x] = 'P';
            }
        }
    }

    map
}