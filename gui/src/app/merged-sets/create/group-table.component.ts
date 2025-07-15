import { Component, Input, input } from '@angular/core';
import { NgFor, NgIf } from '@angular/common';

@Component({
    selector: 'app-merged-sets-group-table',
    standalone: true,
    imports: [
        NgFor,
        NgIf,
    ],
    template: `
        <table class="w-full">
            <tr>
                <th class="bg-gray-500 p-2 rounded-l-md">Groups</th>
            </tr>
            <ng-container *ngIf="groups.length > 0">
                <ng-container *ngFor="let group of groups">
                    <tr>
                        <td class="p-0.5"></td>
                    </tr>
                    <tr>
                        <td class="bg-gray-600 p-2 rounded-md text-center">{{group}}</td>
                    </tr>
                </ng-container>
            </ng-container>

            <ng-container *ngIf="groups.length === 0">
                <tr>
                    <td class="p-0.5"></td>
                </tr>
                <tr>
                    <td class="bg-gray-600 p-2 rounded-md text-center text-gray-300">None</td>
                </tr>
            </ng-container>
        </table>
    `
})
export class MergedSetsGroupTableComponent {
    @Input() groups: string[] = [];

    protected readonly Object = Object;
}

