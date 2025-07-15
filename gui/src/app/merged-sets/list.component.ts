import { Component } from '@angular/core';
import { listMergedDatasets } from "../../backend";
import { NgForOf, NgIf } from "@angular/common";
import { DsmSets, MergedSet } from "../../backend.types";

@Component({
    selector: 'app-merged-sets-list',
    standalone: true,
    imports: [
        NgForOf,
        NgIf
    ],
    template: `
        <h1 class="text-xl">Merged Sets</h1>
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
                        <td class="p-2 rounded-l-md">{{dataset.name}}</td>
                        <td class="p-2 text-center">v{{dataset.version}}</td>
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
export class MergedSetsListComponent {
    datasets: MergedSet[] = [];

    constructor() {
        listMergedDatasets().then((sets) => this.datasets = sets);
    }

    select(sets: MergedSet) {
        window.location.href = `/merged-sets/${sets.name}/${sets.version}`
    }

    formatClasses(sets: MergedSet): string {
        return Object.keys(sets.class_mapper.classes).reduce((acc, val) => {
            if (acc.length > 0)
                acc = acc + ", " + val;
            else
                acc = val;
            return acc;
        }, "");
    }

    countDsm(sets: DsmSets): number {
        return Object.values(sets.entries).reduce((acc, val) => {
            for (const _ of val)
                acc++;
            return acc;
        }, 0);
    }

    countImage(sets: MergedSet): number {
        let count = 0;
        for (const group in sets.datasets) {
            count += sets.datasets[group]
                .map(this.countDsm)
                .reduce((acc, dsm) => acc + dsm);
        }
        return count;
    }

    protected readonly Object = Object;
}
