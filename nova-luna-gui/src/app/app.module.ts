import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppRoutingModule } from "./app-routing.module";
import { AppComponent } from "./app.component";
import { TileComponent } from "./tile/tile.component";
import { GameBoardComponent } from "./game-board/game-board.component";
import { TaskComponent } from "./task/task.component";
import { TileBgColorPipe } from "./tile-bg-color.pipe";

@NgModule({
  declarations: [
    AppComponent,
    TileComponent,
    GameBoardComponent,
    TaskComponent,
    TileBgColorPipe,
  ],
  imports: [BrowserModule, AppRoutingModule],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
