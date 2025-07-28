import { Component } from '@angular/core';
import { Actions, SetInfo } from '../../backend.types';
import { getAllActions } from '../../backend';
import { NgFor, NgIf } from '@angular/common';

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [
    NgFor,
    NgIf
  ],
  template: `
  <div class="flex flex-col bg-gray-800 m-2 p-2 rounded-md">
      <h1 class="text-xl">Recent actions</h1>
      <table class="w-full">
          <tr>
              <th class="bg-gray-600 p-2 rounded-l-md w-1/6 text-left">Action</th>
              <th class="bg-gray-600 p-2 w-2/6 text-left">Name</th>
              <th class="bg-gray-600 p-2 w-1/6">Version</th>
              <th class="bg-gray-600 p-2 w-1/6">Datasets Included</th>
              <th class="bg-gray-600 p-2 rounded-r-md w-1/6">Timestamp</th>
          </tr>
          <ng-container *ngIf="actions.length > 0">
              <ng-container *ngFor="let action of actions">
                  <tr>
                      <td class="p-0.5"></td>
                  </tr>
                  <tr *ngIf="action.action == 'New'" (click)="select(action)" class="bg-green-700 hover:bg-green-600">
                      <td class="p-2 rounded-l-md">{{action.action}}</td>
                      <td class="p-2">{{action.name}}</td>
                      <td class="p-2 text-center">v{{action.version}}</td>
                      <td class="p-2 text-center">-</td>
                      <td class="p-2 text-center rounded-r-md">{{formatTime(action.time)}}</td>
                  </tr>
                  <tr *ngIf="action.action == 'Merge'" (click)="select(action)" class="bg-blue-700 hover:bg-blue-600">
                      <td class="p-2 rounded-l-md">{{action.action}}</td>
                      <td class="p-2">{{action.name}}</td>
                      <td class="p-2 text-center">v{{action.version}}</td>
                      <td class="p-2 text-center">Containing {{action.datasets.length}} datasets</td>
                      <td class="p-2 text-center rounded-r-md">{{formatTime(action.time)}}</td>
                  </tr>
              </ng-container>
          </ng-container>
          <ng-container *ngIf="actions.length === 0">
              <tr>
                  <td class="p-0.5"></td>
              </tr>
              <tr>
                  <td colspan="5" class="bg-gray-700 p-2 rounded-md text-center text-gray-300">None</td>
              </tr>
          </ng-container>
      </table>
  </div>
`
})
export class HomePageComponent {
  actions: Actions[] = []

  constructor() {
    getAllActions().then((res) => this.actions = res);
  }

  select(action: Actions) {
    if (action.datasets.length > 0) {
      window.location.href = `/merged-sets/${action.name}/${action.version}`
    } else {
      window.location.href = `/raw-sets/${action.name}/${action.version}`
    }
  }

  formatTime(date: string) {
    return new Date(date).toLocaleString("en-CA")
  }
}
