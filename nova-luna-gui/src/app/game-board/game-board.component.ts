import { Component, OnInit } from "@angular/core";
import { Observable } from "rxjs";
import { GameBoardService } from "../shared/game-board.service";
import { GameBoardField } from "../shared/game-board-field";
import { map } from "rxjs/operators";

@Component({
  selector: "app-game-board",
  templateUrl: "./game-board.component.html",
  styleUrls: ["./game-board.component.scss"],
})
export class GameBoardComponent implements OnInit {
  constructor(private gameBoardService: GameBoardService) {}

  board$?: Observable<GameBoardField[][]>;

  ngOnInit(): void {
    this.board$ = this.gameBoardService.getGameBoard().pipe(
      map((board) => {
        let minX = 0;
        let minY = 0;
        let maxX = 0;
        let maxY = 0;

        let boardMap = new Map();
        board.forEach((b) => {
          minX = Math.min(minX, b.position[0]);
          minY = Math.min(minY, b.position[1]);
          maxX = Math.max(maxX, b.position[0]);
          maxY = Math.max(maxY, b.position[1]);
          boardMap.set(b.position, b);
        });

        let fullBoard = [];
        for (let x = minX; x <= maxX; ++x) {
          let row = [];
          for (let y = minY; y <= maxY; ++y) {
            let position = [x, y];
            let t = boardMap.get(position);
            if (t != undefined) {
              row.push(t);
            } else {
              row.push({ position, tile: undefined });
            }
          }
          fullBoard.push(row);
        }

        return fullBoard;
      })
    );
  }
}
