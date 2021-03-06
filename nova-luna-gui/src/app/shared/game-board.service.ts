import { Injectable } from "@angular/core";
import { Observable } from "rxjs";
import { of } from "rxjs";
import { map } from "rxjs/operators";
import { GameBoardField } from "./game-board-field";

@Injectable({
  providedIn: "root",
})
export class GameBoardService {
  constructor() {}

  getGameBoard(): Observable<GameBoardField[]> {
    return of(
      JSON.parse(
        `[{"position":[0,0],"tile":{"color":"Yellow","cost":5,"tasks":[{"colors":["Red"],"solved":true},{"colors":["Teal","Teal"],"solved":true}]}},{"position":[0,1],"tile":{"color":"Teal","cost":4,"tasks":[{"colors":["Blue","Red"],"solved":false},{"colors":["Teal","Yellow"],"solved":true}]}},{"position":[1,1],"tile":{"color":"Teal","cost":5,"tasks":[{"colors":["Red","Red"],"solved":false},{"colors":["Blue","Blue"],"solved":true},{"colors":["Teal","Teal"],"solved":true}]}},{"position":[1,0],"tile":{"color":"Teal","cost":4,"tasks":[{"colors":["Yellow","Blue"],"solved":true},{"colors":["Teal","Teal","Teal"],"solved":false}]}},{"position":[2,0],"tile":{"color":"Blue","cost":5,"tasks":[{"colors":["Teal"],"solved":true},{"colors":["Teal","Teal","Teal"],"solved":true}]}},{"position":[2,1],"tile":{"color":"Yellow","cost":6,"tasks":[{"colors":["Blue","Blue","Blue"],"solved":true},{"colors":["Teal","Teal"],"solved":true},{"colors":["Teal","Blue"],"solved":true}]}},{"position":[0,2],"tile":{"color":"Red","cost":7,"tasks":[{"colors":["Teal","Blue"],"solved":true},{"colors":["Teal","Yellow"],"solved":true},{"colors":["Yellow","Blue"],"solved":true}]}},{"position":[-1,2],"tile":{"color":"Yellow","cost":2,"tasks":[{"colors":["Yellow","Yellow","Yellow","Yellow"],"solved":false}]}},{"position":[1,2],"tile":{"color":"Blue","cost":4,"tasks":[{"colors":["Yellow","Yellow"],"solved":false},{"colors":["Teal","Teal","Teal"],"solved":true},{"colors":["Red","Red","Red"],"solved":false}]}},{"position":[2,2],"tile":{"color":"Blue","cost":2,"tasks":[{"colors":["Blue","Blue","Blue","Blue"],"solved":false}]}},{"position":[-1,0],"tile":{"color":"Red","cost":5,"tasks":[{"colors":["Yellow"],"solved":true},{"colors":["Blue","Blue"],"solved":false}]}}]`
      )
    );
  }
}
