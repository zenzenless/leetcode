/*
 * @lc app=leetcode.cn id=37 lang=golang
 *
 * [37] 解数独
 */
package main

// @lc code=start
func solveSudoku(board [][]byte) {

	var points [][2]int
	rows := make([]map[byte]struct{}, 9)
	for r := range board {
		for c := range board[r] {
			if board[r][c] == '.' {
				points = append(points, [2]int{r, c})
			}
		}
		rows[r] = excludeSudu(board[r])
	}
	cols := make([]map[byte]struct{}, 9)
	for c := range board {
		col := make([]byte, 9)
		for r := range board[c] {
			col[r] = board[r][c]
		}
		cols[c] = excludeSudu(col)
	}
	boxs := make(map[[2]int]map[byte]struct{}, 9)
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			box := make([]byte, 0)
			for r := i * 3; r < i*3+3; r++ {
				for c := j * 3; c < j*3+3; c++ {
					box = append(box, board[r][c])
				}
			}
			boxs[[2]int{i, j}] = excludeSudu(box)
		}
	}
	var find func(table [][]byte, r, c int, pointNum int) bool
	//table:临时的表 r,c:当前坐标  pointNum:第n个‘.’
	find = func(table [][]byte, r, c int, pointNum int) bool {
		remains := stillHave(r, c, rows, cols, boxs)
		if len(remains) != 0 && len(points) == pointNum {
			//当前为最后一个点，并且有剩余值
			//设置该点的值
			board[r][c] = remains[0]
			return true
		}
		if len(remains) == 0 {
			return false
		}
		for _, next := range remains {

			//board[r][c] = next
			//find next point
			delete(rows[r], next)
			delete(cols[c], next)
			delete(boxs[[2]int{r / 3, c / 3}], next)
			x, y := points[pointNum][0], points[pointNum][1]
			if found := find(board, x, y, pointNum+1); found {
				board[r][c] = next
				return true
			}
			rows[r][next] = struct{}{}
			cols[c][next] = struct{}{}
			boxs[[2]int{r / 3, c / 3}][next] = struct{}{}

			//board[r][c] = '.'
		}
		return false
	}
	find(board, points[0][0], points[0][1], 1)

}
func excludeSudu(values []byte) map[byte]struct{} {
	nums := map[byte]struct{}{
		'1': {},
		'2': {},
		'3': {},
		'4': {},
		'5': {},
		'6': {},
		'7': {},
		'8': {},
		'9': {},
	}
	//排除同行
	for i := 0; i < 9; i++ {
		delete(nums, values[i])
	}
	return nums
}

func stillHave(r, c int, rows, cols []map[byte]struct{}, boxs map[[2]int]map[byte]struct{}) []byte {
	value := make([]byte, 0)
	tmp := make(map[byte]struct{})
	// for rv := range rows[r] {
	// 	tmp[rv]=struct{}{}
	// }
	for cv := range cols[c] {
		if _, ok := rows[r][cv]; ok {
			tmp[cv] = struct{}{}
		}
	}
	// for bv := range boxs[[2]int{r/3,c/3}] {
	// 	tmp[bv]=struct{}{}
	// }
	for key := range tmp {
		if _, ok := boxs[[2]int{r / 3, c / 3}][key]; ok {
			value = append(value, key)
		}
	}
	//确认九宫格位置
	// rr := 0
	// if r >= 3 {
	// 	rr = 3
	// }
	// if r >= 6 {
	// 	rr = 6
	// }
	// cc := 0
	// if c >= 3 {
	// 	cc = 3
	// }
	// if c >= 6 {
	// 	cc = 6
	// }
	// for i := 0; i < 3; i++ {
	// 	for j := 0; j < 3; j++ {
	// 		delete(nums, board[rr+i][cc+j])
	// 	}
	// }
	return value
}

// @lc code=end
