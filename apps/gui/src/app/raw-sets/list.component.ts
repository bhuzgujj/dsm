import { Component } from '@angular/core';
import { listRawDatasets } from "../../backend";
import { NgForOf, NgIf } from "@angular/common";
import { DsmSets } from "../../backend.types";

@Component({
    selector: 'app-raw-sets-list',
    standalone: true,
    imports: [
        NgForOf,
        NgIf
    ],
    template: `
        <h1 class="text-xl">Raw Sets</h1>
        <table class="w-full">
            <tr>
                <th class="bg-gray-600 p-2 rounded-l-md w-2/6 text-left">Name</th>
                <th class="bg-gray-600 p-2 w-1/6">Version</th>
                <th class="bg-gray-600 p-2 w-1/6">Images</th>
                <th class="bg-gray-600 p-2 rounded-r-md w-2/6">Classes</th>
            </tr>
            <ng-container *ngIf="datasets.length > 0">
                <ng-container *ngFor="let dataset of datasets">
                    <tr>
                        <td class="p-0.5"></td>
                    </tr>
                    <tr (click)="select(dataset)" class="bg-gray-700 hover:bg-gray-600">
                        <td class="p-2 rounded-l-md">{{dataset.metadata.name}}</td>
                        <td class="p-2 text-center">v{{dataset.metadata.version}}</td>
                        <td class="p-2 text-center">
                            {{countImage(dataset)}}
                        </td>
                        <td class="p-2 text-center rounded-r-md">
                            {{formatClasses(dataset)}}
                        </td>
                    </tr>
                </ng-container>
            </ng-container>
            <ng-container *ngIf="datasets.length === 0">
                <tr>
                    <td class="p-0.5"></td>
                </tr>
                <tr>
                    <td colspan="4" class="bg-gray-700 p-2 rounded-md text-center text-gray-300">None</td>
                </tr>
            </ng-container>
        </table>
    `
})
export class RawSetsListComponent {
    datasets: DsmSets[] = [];

    constructor() {
        listRawDatasets().then((sets) => this.datasets = sets);
    }

    select(sets: DsmSets) {
        window.location.href = `/raw-sets/${sets.metadata.name}/${sets.metadata.version}`
    }

    formatClasses(sets: DsmSets): string {
        return Object.values(sets.metadata.classes).reduce((acc, val) => {
            if (acc.length > 0)
                acc = acc + ", " + val.class;
            else
                acc = val.class;
            if (val.subclass)
                acc = acc + val.class + "(" + val.subclass + ")";
            return acc;
        }, "");
    }

    countImage(sets: DsmSets): number {
        return Object.values(sets.entries).reduce((acc, val) => {
            for (const _ of val)
                acc++;
            return acc;
        }, 0);
    }

    protected readonly Object = Object;
}
