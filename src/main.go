package main

import (
	"encoding/json"
	"fmt"
	"log"
)

func main() {
	board := [][]string{}
	err := json.Unmarshal([]byte(bstr), &board)
	if err != nil {
		log.Fatal(err)
	}
	b:=make([][]byte,9)
	for c:=range board{
		for j:=range board[c]{
			b[c]=append(b[c], byte(board[c][j][0]))
		}
	}
	solveSudoku(b)
	for r:=range b{
		for c:=range b{
			fmt.Printf("%s ",string(b[r][c]))

		}
		fmt.Println()
	}
}

var bstr = `[
    [
        "5",
        "3",
        ".",
        ".",
        "7",
        ".",
        ".",
        ".",
        "."
    ],
    [
        "6",
        ".",
        ".",
        "1",
        "9",
        "5",
        ".",
        ".",
        "."
    ],
    [
        ".",
        "9",
        "8",
        ".",
        ".",
        ".",
        ".",
        "6",
        "."
    ],
    [
        "8",
        ".",
        ".",
        ".",
        "6",
        ".",
        ".",
        ".",
        "3"
    ],
    [
        "4",
        ".",
        ".",
        "8",
        ".",
        "3",
        ".",
        ".",
        "1"
    ],
    [
        "7",
        ".",
        ".",
        ".",
        "2",
        ".",
        ".",
        ".",
        "6"
    ],
    [
        ".",
        "6",
        ".",
        ".",
        ".",
        ".",
        "2",
        "8",
        "."
    ],
    [
        ".",
        ".",
        ".",
        "4",
        "1",
        "9",
        ".",
        ".",
        "5"
    ],
    [
        ".",
        ".",
        ".",
        ".",
        "8",
        ".",
        ".",
        "7",
        "9"
    ]
]`
