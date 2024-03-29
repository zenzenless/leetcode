/*
 * @lc app=leetcode.cn id=1976 lang=rust
 * @lcpr version=30117
 *
 * [1976] 到达目的地的方案数
 */

// @lcpr-template-start



// @lcpr-template-end
// @lc code=start
use std::{cmp::Ordering, collections::BinaryHeap};
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n=n as usize;
        //方便查询
        let mut road_map = vec![vec![]; n as usize];
        for r in roads {
            road_map[r[0] as usize].push((r[1] as usize, r[2] as usize));
            road_map[r[1] as usize].push((r[0] as usize, r[2] as usize));
        }
        //经过这个点的路径,只保留最小经过时间的路径 map_wp= [](时间,路径条数)
        let mut map_wp: Vec<((usize, usize))> = vec![(0, 0); n as usize];
        map_wp[0]=(0,1);
        let first = wp {
            current: 0,
            // walk: vec![false; n as usize],
            time: 0,
        };
        let mut wait=BinaryHeap::new();
         //let mut wait = vec![];
        wait.push(first);
        while let Some(way) = wait.pop() {
            let last = way.current;
            //如果当前路径是最后的一个节点，则跳过
            if last==n-1{
                continue;
            }

            //如果到达当前位置的时间比记录的大，则跳过
            let way_to_here_min_time=&map_wp[last].0;
            let way_nums_to_here=map_wp[last].1;
            if *way_to_here_min_time!=0&&*way_to_here_min_time<way.time{
                continue;
            }

            let next_nodes= {
                let to = &road_map[last];
                to.iter()
            };
            //去往下一个点，且时间是所有路径中最少的
            for (next,to_next_node_time) in next_nodes{
                if *next==0{
                    continue;
                }
                //从0点开始，去到下一个点的时间 = 到达这里的时间+去下一个节点的时间
                let access_next_node_time=way.time+to_next_node_time;
                //经过该点的路径
                let ways_to_next=&mut map_wp[*next];

                //如果最小路径时间相同，则相加
                if ways_to_next.0==access_next_node_time{

                    ways_to_next.1+=way_nums_to_here;
                    ways_to_next.1=ways_to_next.1%1_000_000_007;
                    
                    continue;
                }

                //如果是比其它路径更少的时间，路径条数为当前条数，记录时间
                if ways_to_next.0==0||ways_to_next.0>access_next_node_time{
                    ways_to_next.1=way_nums_to_here;
                    ways_to_next.0=access_next_node_time;
                    let mut next_wp=way.clone();
                    next_wp.current=*next;
                    next_wp.time=access_next_node_time;
                    wait.push(next_wp);
                }

                
            }
        }
        let (min_time,way_to_end)=&mut map_wp[n-1];
        *way_to_end as i32

    }
}

#[derive(Clone,PartialEq, Eq,Ord)]
struct wp {
    //当前位置
    current: usize,
    //当前花费的时间
    time: usize,
}
impl PartialOrd for wp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.time.cmp(&self.time))
    }
}
// @lc code=end

struct Solution;

#[test]
fn test_func() {
    let roads=vec![vec![1,0,9611],vec![0,2,13741],vec![2,1,4130],vec![2,3,2339],vec![3,0,16080],vec![1,4,9725],vec![0,4,19336],vec![3,4,3256],vec![4,2,5595],vec![5,4,6224],vec![6,2,1303],vec![1,6,5433],vec![7,6,10824],vec![4,7,6532],vec![5,7,308],vec![7,1,16257],vec![6,8,14961],vec![8,4,10669],vec![8,0,30005],vec![5,8,4445],vec![8,3,13925],vec![8,7,4137],vec![2,8,16264],vec![9,4,12915],vec![0,9,32251],vec![8,9,2246],vec![10,7,14204],vec![0,10,40072],vec![6,10,25028],vec![10,8,10067],vec![10,3,23992],vec![10,2,26331],vec![10,1,30461],vec![4,10,20736],vec![5,10,14512],vec![9,10,7821],vec![11,4,3381],vec![12,4,27123],vec![9,12,14208],vec![10,12,6387],vec![8,12,16454],vec![12,0,46459],vec![7,12,20591],vec![12,5,20899],vec![2,12,32718],vec![12,11,23742],vec![1,12,36848],vec![6,12,31415],vec![5,13,25075],vec![13,10,10563],vec![3,13,34555],vec![13,12,4176],vec![13,8,20630],vec![13,1,41024],vec![13,11,27918],vec![13,7,24767],vec![4,13,31299],vec![2,13,36894],vec![10,14,8784],vec![12,14,2397],vec![4,14,29520],vec![6,14,33812],vec![9,14,16605],vec![14,3,32776],vec![5,14,23296],vec![14,2,35115],vec![8,14,18851],vec![7,14,22988],vec![10,15,9236],vec![15,3,33228],vec![15,0,49308],vec![15,12,2849],vec![4,16,41221],vec![16,8,30552],vec![10,16,20485],vec![16,11,37840],vec![16,6,45513],vec![16,14,11701],vec![3,16,44477],vec![1,16,50946],vec![16,5,34997],vec![16,7,34689],vec![12,16,14098],vec![16,0,60557],vec![16,13,9922],vec![2,16,46816],vec![16,9,28306],vec![17,9,36735],vec![17,3,52906],vec![16,17,8429],vec![8,17,38981],vec![7,17,43118],vec![6,17,53942],vec![4,17,49650],vec![17,14,20130],vec![17,13,18351],vec![17,10,28914],vec![17,11,46269],vec![1,17,59375],vec![15,17,19678],vec![17,12,22527],vec![15,18,27895],vec![18,12,30744],vec![18,11,54486],vec![18,4,57867],vec![3,18,61123],vec![18,16,16646],vec![13,18,26568],vec![18,8,47198],vec![1,18,67592],vec![17,18,8217],vec![0,18,77203],vec![6,18,62159],vec![18,14,28347],vec![19,13,32225],vec![11,19,60143],vec![5,19,57300],vec![19,15,33552],vec![10,19,42788],vec![6,19,67816],vec![7,19,56992],vec![19,18,5657],vec![19,1,73249],vec![16,19,22303],vec![8,19,52855],vec![17,19,13874],vec![19,3,66780],vec![19,9,50609],vec![19,0,82860],vec![19,4,63524],vec![4,20,69122],vec![18,20,11255],vec![3,20,72378],vec![11,20,65741],vec![14,20,39602],vec![10,20,48386],vec![1,20,78847],vec![20,5,62898],vec![20,15,39150],vec![20,19,5598],vec![16,20,27901],vec![12,20,41999],vec![0,20,88458],vec![8,20,58453],vec![20,13,37823],vec![20,7,62590],vec![9,20,56207],vec![2,20,74717],vec![20,17,19472],vec![17,21,26673],vec![16,21,35102],vec![3,21,79579],vec![21,18,18456],vec![21,9,63408],vec![21,20,7201],vec![2,21,81918],vec![21,11,72942],vec![14,22,50771],vec![22,19,16767],vec![22,18,22424],vec![22,15,50319],vec![22,13,48992],vec![22,3,83547],vec![6,22,84583],vec![22,5,74067],vec![22,10,59555],vec![16,22,39070],vec![22,20,11169],vec![22,12,53168],vec![4,22,80291],vec![22,2,85886],vec![22,8,69622],vec![22,21,3968],vec![22,17,30641],vec![0,22,99627],vec![11,22,76910],vec![22,7,73759],vec![2,23,87059],vec![23,6,85756],vec![5,23,75240],vec![23,21,5141],vec![9,23,68549],vec![14,23,51944],vec![20,23,12342],vec![1,23,91189],vec![8,23,70795],vec![11,23,78083],vec![23,13,50165],vec![23,22,1173],vec![12,23,54341],vec![23,3,84720],vec![23,10,60728],vec![23,17,31814],vec![21,24,5622],vec![14,24,52425],vec![24,20,12823],vec![7,24,75413],vec![24,2,87540],vec![25,1,101404],vec![13,25,60380],vec![25,7,85147],vec![9,25,78764],vec![15,25,61707],vec![19,25,28155],vec![25,22,11388],vec![25,17,42029],vec![25,5,85455],vec![16,25,50458],vec![25,4,91679],vec![25,23,10215],vec![25,14,62159],vec![24,25,9734],vec![25,21,15356],vec![25,10,70943],vec![25,20,22557],vec![3,25,94935],vec![0,25,111015],vec![19,26,31787],vec![14,26,65791],vec![26,3,98567],vec![26,15,65339],vec![26,25,3632],vec![24,26,13366],vec![9,26,82396],vec![18,26,37444],vec![26,17,45661],vec![26,1,105036],vec![22,26,15020],vec![26,5,89087],vec![10,26,74575],vec![26,2,100906],vec![11,26,91930],vec![13,26,64012],vec![26,12,68188],vec![26,4,95311],vec![20,26,26189],vec![0,26,114647],vec![26,21,18988],vec![0,27,100248],vec![21,27,4589],vec![2,27,86507],vec![4,27,80912],vec![27,9,67997],vec![14,27,51392],vec![27,15,50940],vec![27,10,60176],vec![27,11,77531],vec![27,13,49613],vec![0,28,124384],vec![28,12,77925],vec![28,27,24136],vec![28,26,9737],vec![28,10,84312],vec![28,13,73749],vec![28,16,63827],vec![20,28,35926],vec![28,23,23584],vec![29,6,100376],vec![29,23,14620],vec![29,17,46434],vec![29,25,4405],vec![22,29,15793],vec![29,15,66112],vec![5,29,89860],vec![0,29,115420],vec![13,29,64785],vec![29,4,96084],vec![29,19,32560],vec![29,21,19761],vec![29,26,773],vec![11,29,92703],vec![9,29,83169],vec![29,18,38217],vec![29,10,75348],vec![7,29,89552],vec![1,29,105809],vec![29,20,26962],vec![29,16,54863],vec![12,29,68961],vec![29,2,101679],vec![29,24,14139],vec![4,30,111360],vec![10,30,90624],vec![30,5,105136],vec![30,1,121085],vec![8,30,100691],vec![28,30,6312],vec![27,30,30448],vec![30,24,29415],vec![30,26,16049],vec![30,12,84237],vec![6,30,115652],vec![30,2,116955],vec![30,14,81840],vec![30,20,42238],vec![30,29,15276],vec![30,9,98445],vec![30,3,114616],vec![30,16,70139],vec![21,30,35037],vec![30,25,19681],vec![30,13,80061],vec![18,30,53493],vec![30,11,107979],vec![30,15,81388],vec![30,0,130696],vec![31,16,58739],vec![31,29,3876],vec![6,31,104252],vec![31,2,105555],vec![31,15,69988],vec![1,32,117525],vec![20,32,38678],vec![7,32,101268],vec![27,32,26888],vec![25,32,16121],vec![29,32,11716],vec![23,32,26336],vec![32,14,78280],vec![31,32,7840],vec![15,32,77828],vec![32,5,101576],vec![11,32,104419],vec![12,32,80677],vec![23,33,30143],vec![18,33,53740],vec![33,10,90871],vec![31,33,11647],vec![33,21,35284],vec![33,25,19928],vec![33,32,3807],vec![33,26,16296]]
;
    let p = Solution::count_paths(
        34,
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
    assert_eq!(6347247,p)
}

#[test]
fn test_ord(){
    let mut wait=BinaryHeap::new();
     //let mut wait = vec![];
    wait.push(wp {
        current: 0,
        // walk: vec![false; n as usize],
        time: 0,
    });

    wait.push(wp {
        current: 0,
        // walk: vec![false; n as usize],
        time: 3,
    });


    wait.push(wp {
        current: 0,
        // walk: vec![false; n as usize],
        time: 1,
    });

    wait.push(wp {
        current: 0,
        // walk: vec![false; n as usize],
        time: 4,
    });
    println!("{:?}",wait.peek().unwrap().time)
}
/*
// @lcpr case=start
// 7\n[[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]\n
// @lcpr case=end

// @lcpr case=start
// 2\n[[1,0,10]]\n
// @lcpr case=end

 */
