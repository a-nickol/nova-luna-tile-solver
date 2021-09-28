import { Component, Input, OnInit } from '@angular/core';
import { Task } from '../shared/task';

@Component({
  selector: 'app-task',
  templateUrl: './task.component.html',
  styleUrls: ['./task.component.scss']
})
export class TaskComponent {

  @Input()
  task?: Task;

  column(i: number): string {
    let colors = this.task?.colors;
    if (colors != undefined && colors.length % 2 == 1 && i == 0) {
      return "col-span-2";
    }
    return "";
  }
}
