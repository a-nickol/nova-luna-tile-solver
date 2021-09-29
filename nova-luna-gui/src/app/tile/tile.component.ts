import { Component, Input } from "@angular/core";
import { GameBoardField } from "../shared/game-board-field";
import { Task } from "../shared/task";

@Component({
  selector: "app-tile",
  templateUrl: "./tile.component.html",
  styleUrls: ["./tile.component.scss"],
})
export class TileComponent {
  @Input()
  tile?: GameBoardField;

  getTask(num: number): Task | undefined {
    let tasks = this.tile?.tile?.tasks;
    if (tasks) {
      let len = tasks.length;
      if (num >= 0 && num < len) {
        return tasks[num];
      }
    }
    return undefined;
  }
}
