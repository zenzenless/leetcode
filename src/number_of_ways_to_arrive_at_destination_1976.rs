/*
 * @lc app=leetcode.cn id=1976 lang=rust
 * @lcpr version=30117
 *
 * [1976] 到达目的地的方案数
 */

// @lcpr-template-start

use std::cmp::Ordering;

// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        //方便查询
        let mut road_map = vec![vec![]; n as usize];
        for r in roads {
            road_map[r[0] as usize].push((r[1], r[2]));
            road_map[r[1] as usize].push((r[0], r[2]));
        }
        //经过这个点的路径,只保留最小经过时间的路径 map_wp= [](time,[]wp)
        let mut map_wp: Vec<((i32, Vec<wp>))> = vec![(0, vec![]); n as usize];
        let first = wp {
            path: vec![0],
            walk: vec![0; n as usize],
            time: 0,
        };
        let mut wait = vec![first];
        while let Some(way) = wait.pop() {
            let last = way.path.last().unwrap();

            //如果当前路径是最后的一个节点，则跳过
            if *last==n-1{
                continue;
            }
            let next_nodes: Vec<&(i32, i32)> = {
                let to = &road_map[*last as usize];
                //过滤出这条路径还没去过的地方
                to.iter()
                    .filter(|&x| x.0 != 0 && way.walk[x.0 as usize] == 0)
                    .collect()
            };
            //去往下一个点，且时间是所有路径中最少的
            for (next,to_next_node_time) in next_nodes{
                //从0点开始，去到下一个点的时间 = 到达这里的时间+去下一个节点的时间
                let access_next_node_time=way.time+to_next_node_time;
                //经过该点的路径
                let ways_to_next=&mut map_wp[*next as usize];

                //添加到全局路径记录 添加到待继续走路径
                //如果到达下一个节点的时间是所有路径中最少的话，则添加到记录
                if ways_to_next.0==0||ways_to_next.0>=access_next_node_time{
                    let mut next_wp=way.clone();
                    next_wp.path.push(*next);
                    next_wp.walk[*next as usize]=access_next_node_time;
                    next_wp.time=access_next_node_time;
                    wait.push(next_wp.clone());
                    ways_to_next.1.push(next_wp);
                    ways_to_next.0=access_next_node_time;
                    //TODO: 删除时间不是最少的路径： way_to_here
                }
            }
        }
        let (min_time,way_to_end)=&mut map_wp[n as usize -1];
        let less_time_ways=way_to_end.iter().filter(|&x|x.time==*min_time).count();
        //println!("{:?}",&less_time_ways);
        less_time_ways as i32

    }
}
//路径
#[derive(Clone)]
struct wp {
    //路径经过的点的顺序
    path: Vec<i32>,
    //这条路径经过的地方,以及经过时花费的时间
    walk: Vec<i32>,
    //当前花费的时间
    time: i32,
}
// @lc code=end

struct Solution;

#[test]
fn test_func() {
    let roads=vec![vec![1,0,2423],vec![0,2,3581],vec![3,1,6841],vec![4,0,13171],vec![1,4,10748],vec![3,4,3907],vec![2,4,9590],vec![1,5,17614],vec![2,5,16456],vec![5,0,20037],vec![5,4,6866],vec![5,3,10773],vec![2,6,19563],vec![6,0,23144],vec![4,6,9973],vec![7,6,8188],vec![5,7,11295],vec![7,4,18161],vec![7,1,28909],vec![8,7,4490],vec![8,2,32241],vec![1,8,33399],vec![5,8,15785],vec![0,8,35822],vec![8,4,22651],vec![8,3,26558],vec![8,6,12678],vec![8,9,7933],vec![9,5,23718],vec![10,8,8288],vec![10,6,20966],vec![3,10,34846],vec![9,10,355],vec![10,2,40529],vec![10,5,24073],vec![10,7,12778],vec![10,4,30939],vec![10,1,41687],vec![0,10,44110],vec![7,11,7876],vec![1,11,36785],vec![0,11,39208],vec![12,11,9566],vec![3,12,39510],vec![10,12,4664],vec![12,2,45193],vec![12,6,25630],vec![5,12,28737],vec![8,12,12952],vec![13,2,45358],vec![13,12,165],vec![8,13,13117],vec![13,9,5184],vec![13,5,28902],vec![13,4,35768],vec![3,13,39675],vec![7,13,17607],vec![14,11,11418],vec![8,14,14804],vec![14,9,6871],vec![14,2,47045],vec![13,14,1687],vec![14,0,50626],vec![15,5,33618],vec![15,10,9545],vec![15,14,3029],vec![15,1,51232],vec![15,9,9900],vec![15,0,53655],vec![3,15,44391],vec![4,15,40484],vec![12,15,4881],vec![7,15,22323],vec![6,15,30511],vec![0,16,53825],vec![1,16,51402],vec![12,16,5051],vec![16,10,9715],vec![13,16,4886],vec![11,16,14617],vec![16,4,40654],vec![16,8,18003],vec![16,14,3199],vec![2,16,50244],vec![11,17,750],vec![17,3,30694],vec![0,17,39958],vec![14,18,9823],vec![18,3,51185],vec![17,18,20491],vec![18,10,16339],vec![15,19,14284],vec![5,19,47902],vec![11,19,28731],vec![19,8,32117],vec![12,19,19165],vec![1,19,65516],vec![17,19,27981],vec![18,19,7490],vec![14,19,17313],vec![19,9,24184],vec![20,19,4509],vec![20,1,70025],vec![20,10,28338],vec![6,20,49304],vec![18,20,11999],vec![8,20,36626],vec![2,20,68867],vec![20,7,41116],vec![20,13,23509],vec![3,20,63184],vec![20,21,3222],vec![21,19,7731],vec![6,21,52526],vec![22,17,12982],vec![0,22,52940],vec![22,9,9185],vec![23,15,3926],vec![11,23,18373],vec![23,12,8807],vec![17,23,17623],vec![10,23,13471],vec![22,24,9868],vec![10,24,18698],vec![9,24,19053],vec![16,24,8983],vec![24,6,39664],vec![17,24,22850],vec![24,12,14034],vec![18,25,9255],vec![0,25,69704],vec![24,25,6896],vec![15,25,16049],vec![25,9,25949],vec![6,25,46560],vec![26,21,6388],vec![17,26,42100],vec![3,26,72794],vec![11,26,42850],vec![26,18,21609],vec![26,9,38303],vec![27,0,20849],vec![27,4,7678],vec![28,2,87143],vec![19,28,22785],vec![28,3,81460],vec![8,28,54902],vec![28,21,15054],vec![28,27,69875],vec![28,17,50766],vec![28,4,77553],vec![28,23,33143],vec![15,28,37069],vec![5,28,70687],vec![26,28,8666],vec![18,28,30275],vec![28,16,36899],vec![10,28,46614],vec![22,28,37784],vec![6,28,67580],vec![28,24,27916],vec![20,28,18276],vec![28,12,41950],vec![28,13,41785],vec![28,0,90724],vec![28,14,40098],vec![28,7,59392],vec![28,9,46969],vec![29,21,23540],vec![29,6,76066],vec![29,12,50436],vec![24,29,36402],vec![29,17,59252],vec![3,29,89946],vec![8,29,63388],vec![10,29,55100],vec![29,4,86039],vec![19,29,31271],vec![5,29,79173],vec![13,29,50271],vec![29,22,46270],vec![25,29,29506],vec![20,29,26762],vec![29,15,45555],vec![29,14,48584],vec![16,29,45385],vec![29,11,60002],vec![28,29,8486],vec![29,1,96787],vec![29,23,41629],vec![30,10,49393],vec![0,30,93503],vec![30,8,57681],vec![30,1,91080],vec![30,24,30695],vec![30,27,72654],vec![26,30,11445],vec![30,28,2779],vec![23,30,35922],vec![31,5,87584],vec![12,31,58847],vec![31,27,86772],vec![31,22,54681],vec![31,29,8411],vec![17,31,67663],vec![3,31,98357],vec![31,15,53966],vec![18,31,47172],vec![31,21,31951],vec![31,19,39682],vec![2,31,104040],vec![31,8,71799],vec![31,30,14118],vec![26,31,25563],vec![0,31,107621],vec![31,13,58682],vec![31,6,84477],vec![14,31,56995],vec![1,31,105198],vec![31,10,63511],vec![31,28,16897],vec![31,25,37917],vec![32,20,4387],vec![10,32,32725],vec![32,6,53691],vec![3,32,67571]];
    let p = Solution::count_paths(
        33,
        // vec![
        //     vec![0, 6, 7],
        //     vec![0, 1, 2],
        //     vec![1, 2, 3],
        //     vec![1, 3, 3],
        //     vec![6, 3, 3],
        //     vec![3, 5, 1],
        //     vec![6, 5, 1],
        //     vec![2, 5, 1],
        //     vec![0, 4, 5],
        //     vec![4, 6, 2],
        // ],
        roads
    );
    println!("{p}")
}
/*
// @lcpr case=start
// 7\n[[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]\n
// @lcpr case=end

// @lcpr case=start
// 2\n[[1,0,10]]\n
// @lcpr case=end

 */
