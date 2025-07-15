import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [
    RouterOutlet,
  ],
  template: `
  <div class="flex flex-col bg-gray-800 m-2 p-2 rounded-md">
      <h1>TODO</h1>
      <router-outlet></router-outlet>
  </div>
`
})
export class HomePageComponent {
}
