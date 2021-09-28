import { Component, OnInit } from '@angular/core';
import { Observable } from 'rxjs';
import { GameBoardService } from '../shared/game-board.service';
import { PlayedTile } from '../shared/played-tile';
import { map } from 'rxjs/operators';

@Component({
  selector: 'app-game-board',
  templateUrl: './game-board.component.html',
  styleUrls: ['./game-board.component.scss']
})
export class GameBoardComponent implements OnInit {

  constructor(private gameBoardService: GameBoardService) { };

  board$?: Observable<PlayedTile[]>;

  ngOnInit(): void {
    this.board$ = this.gameBoardService.getGameBoard().pipe(
      map(board => {
        let x = 0;
        let y = 0;
        board.forEach(b => {
          x = Math.min(x, b.position[0]);
          y = Math.min(y, b.position[1]);
        });

        board.forEach(b => {
          if (x < 0) {
            b.position[0] -= x;
          }
          if (y < 0) {
            b.position[1] -= y;
          }
        });

        return board;
      })
    );
  }
}
