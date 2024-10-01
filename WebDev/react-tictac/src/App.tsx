import { useState } from 'react'
import './App.css'

function Square({value, onSquareClick}: any) {
  return (
    <button
      className="flex items-center justify-center w-16 h-16 bg-gray-200 text-gray-800 border border-gray-400 rounded-lg"
      onClick={onSquareClick}
    >
      {value}
    </button>
  );
}

function Board({ xIsNext, squares, onPlay }: any) {

  function handleClick(i: number) {
    if (squares[i] || calculateWinner(squares)) {
      return;
    }

    const nextSquares = squares.slice();
    if (xIsNext) {
      nextSquares[i] = "X";
    } else {
      nextSquares[i] = "O";
    }
    
    onPlay(nextSquares);
  }

  const winner = calculateWinner(squares);
  let status;
  if (winner) {
    status = `Winner: ${winner}`;
  } else {
    status = `Next player: ${xIsNext? "X" : "O"}`;
  }

  return (
    <>
      <div className="status">{status}</div>
      <div className="grid grid-cols-3 gap-x-2 gap-y-4">
        <Square value={squares[0]} onSquareClick={() => handleClick(0)} />        
        <Square value={squares[1]} onSquareClick={() => handleClick(1)} />        
        <Square value={squares[2]} onSquareClick={() => handleClick(2)} />        
      </div>
      <div className="grid grid-cols-3 gap-x-2 gap-y-4">
        <Square value={squares[4]} onSquareClick={() => handleClick(4)} />        
        <Square value={squares[5]} onSquareClick={() => handleClick(5)} />        
        <Square value={squares[6]} onSquareClick={() => handleClick(6)} />        
      </div>
      <div className="grid grid-cols-3 gap-x-2 gap-y-4">
        <Square value={squares[7]} onSquareClick={() => handleClick(7)} />        
        <Square value={squares[8]} onSquareClick={() => handleClick(8)} />        
        <Square value={squares[9]} onSquareClick={() => handleClick(9)} />        
      </div>
    </>
  );
}

export default function Game() {
  const [history, setHistory] = useState([Array(9).fill(null)]);
  const [currentMove, setCurrentMove] = useState(0);
  const xIsNext = currentMove % 2 === 0;
  const currentSquares = history[currentMove];

  function handlePlay(nextSquares: any) {
    const nextHistory = [...history.slice(0, currentMove + 1), nextSquares]
    setHistory(nextHistory);
    setCurrentMove(nextHistory.length - 1);
  }

  function jumpTo(nextMove: any) {
    setCurrentMove(nextMove);
  }

  const moves = history.map((_, move) => {
      let description;
      if (move > 0) {
        description = `Go to move #${move}`;
      } else {
        description = 'Go to game start';
      }
      return (
        <li key={move}><button onClick={() => jumpTo(move)}>
          {description}
        </button></li>
      );
    }
  )

  return (
    <>
      <div className="game">
        <div className="game-board">
          <Board xIsNext={xIsNext} squares={currentSquares} onPlay={handlePlay}/>
        </div>
        <div className="game-info">
          <ol>{moves}</ol>
        </div>
      </div>
    </>
  )
}

function calculateWinner(squares: Array<null|string>): null|string {
  const lines = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6]
  ];
  for (let i = 0; i < lines.length; i++) {
    const [a, b, c] = lines[i];
    if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
      return squares[a];
    }
  }
  return null;
}
