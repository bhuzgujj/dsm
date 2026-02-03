import { Component } from '@angular/core';
import { NgFor, NgIf } from "@angular/common";
import { MergedSet } from "../../backend.types";
import { FormsModule } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { generateMergedSet, getMergedSet } from '../../backend';
import { open } from '@tauri-apps/plugin-dialog';

type ClassCount = {
    class: string,
    subclass: string | null,
    count: number
};

@Component({
    selector: 'app-merged-sets-details',
    standalone: true,
    imports: [
        NgIf,
        NgFor,
        FormsModule
    ],
    template: `
        <div class="w-full flex flex-col bg-gray-700 p-2 rounded-md">
            <ng-container *ngIf="datasets === undefined">
                <h1 class="text-xl">Loading...</h1>
            </ng-container>
            <ng-container *ngIf="datasets === null">
                <h1 class="text-xl">Not found</h1>
            </ng-container>
            <ng-container *ngIf="datasets !== null && datasets !== undefined">
                <h1 class="text-xl">Details</h1>
                <div class="flex flex-row w-full">
                    <div class="flex flex-col w-full mr-1">
                        <label class="text-gray-400 italic">Name</label>
                        <input [(ngModel)]="name" disabled class="bg-gray-800 p-2 rounded-md" type="text" />
                        <br>
                        <label class="text-gray-400 italic">Version</label>
                        <input [(ngModel)]="version" disabled class="bg-gray-800 p-2 rounded-md" type="text" />
                        <br>
                        <button (click)="generate()" class="bg-green-600 hover:bg-green-300 p-2 rounded-md">Generate</button>
                    </div>
                    <div class="flex flex-col w-full ml-1">
                        <table class="w-full">
                            <tr>
                                <th class="bg-gray-500 p-2 rounded-l-md w-2/5 text-left">Class</th>
                                <th class="bg-gray-500 p-2 w-1/5">Subclass</th>
                                <th class="bg-gray-500 p-2 rounded-r-md w-2/5">Count</th>
                            </tr>
                            <ng-container *ngIf="annotations.length > 0">
                                <ng-container *ngFor="let annotation of annotations">
                                    <tr>
                                        <td class="p-0.5"></td>
                                    </tr>
                                    <tr class="bg-gray-600">
                                        <td class="p-2 rounded-l-md">{{annotation.class}}</td>
                                        <td *ngIf="annotation.subclass" class="p-2 text-center">{{annotation.subclass}}</td>
                                        <td *ngIf="!annotation.subclass" class="p-2 text-center italic text-gray-500">None</td>
                                        <td class="p-2 text-center rounded-r-md">{{annotation.count}}</td>
                                    </tr>
                                </ng-container>
                            </ng-container>
                            <ng-container *ngIf="annotations.length === 0">
                                <tr>
                                    <td colspan="3" class="bg-gray-700 p-2 rounded-md text-center text-gray-300">None</td>
                                </tr>
                            </ng-container>
                        </table>
                    </div>
                </div>
            </ng-container>
        </div>
    `
})
export class MergedSetsDetailsComponent {
    datasets: MergedSet | null | undefined = undefined;
    name = "";
    version = "";
    annotations: ClassCount[] = [];

    constructor(route: ActivatedRoute) {
        route.paramMap.subscribe((params) => {
            let name = params.get('name')!;
            let version = params.get('version')!;
            getMergedSet(name, version)
                .then((ds) => {
                    this.datasets = ds;
                    if (ds) {
                        this.extractDataset(ds);
                    }
                })
                .catch((err) => {
                    this.datasets = null;
                    console.error(err);
                });
        });
    }

    generate() {
        open({
            multiple: false,
            directory: true
        })
            .then((path) => generateMergedSet(this.name, this.version, "yolo-1-1", path!))
            .then(() => window.location.href = "/merged-sets")
            .catch(console.error)
    }

    private extractDataset(ds: MergedSet) {
        this.name = ds.name;
        this.version = ds.version;
        for (const classes of Object.keys(ds.class_mapper.classes)) {
            const index = ds.class_mapper.classes[classes]
            this.annotations[index] = {
                class: classes,
                subclass: null,
                count: 0
            };
        }
        for (const group of Object.values(ds.datasets)) {
            for (const dsm of group) {
                for (const subset in dsm.entries) {
                    for (const entry of dsm.entries[subset]) {
                        for (const annotation of entry.annotation) {
                            const classKey = dsm.metadata.classes[annotation.class].class;
                            let mappedKey = null;
                            for (const keys in ds.class_mapper.mapping) {
                                if (ds.class_mapper.mapping[keys].includes(classKey)) {
                                    mappedKey = keys;
                                    break;
                                }
                            }
                            const mappedIndex = ds.class_mapper.classes[mappedKey!];
                            this.annotations[mappedIndex].count++;
                        }
                    }
                }
            }
        }
    }
}
