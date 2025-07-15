import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ClassMapper, DsmSets, IncludedSet, MergedSet } from '../../../backend.types';
import { listMergedDatasets, listRawDatasets, mergeNew } from '../../../backend';
import { NgFor, NgIf } from '@angular/common';
import { MergedSetsGroupTableComponent } from './group-table.component';

type Mapping = {
    from: string,
    to: string | null
}

@Component({
    selector: 'app-merged-sets-create',
    standalone: true,
    imports: [
        MergedSetsGroupTableComponent,
        FormsModule,
        NgFor,
        NgIf,
    ],
    template: `
        <div class="w-full flex flex-col bg-gray-700 p-2 rounded-md">
            <h1 class="text-xl">Create Merged Sets</h1>
            <label class="text-gray-400 italic">Base Merged Set</label>
            <select [(ngModel)]="mergedSets" class="bg-gray-900 p-2 rounded-md">
                <option [ngValue]="null" class="text-gray-400 italic">Create new set</option>
                <option *ngFor="let merged of mergedSets" [ngValue]="merged">{{merged.name}}
                    v{{merged.version}}
                </option>
            </select>
            <br>

            <label class="text-gray-400 italic">Name</label>
            <input [(ngModel)]="name" class="bg-gray-900 p-2 rounded-md" type="text" />
            <br>

            <label class="text-gray-400 italic">Version</label>
            <input [(ngModel)]="version" class="bg-gray-900 p-2 rounded-md" type="text" />
            <br>

            <h1 class="text-xl">Included sets</h1>
            <div class="flex flex-row w-full">
                <div class="flex flex-col w-full ml-1">
                    <label class="text-gray-400 italic">Add groups</label>
                    <div class="flex">
                        <button (click)="addGroup(inputGroup)"
                            class="bg-green-600 hover:bg-green-300 p-2 rounded-md">Add</button>
                        <input [(ngModel)]="inputGroup" class="bg-gray-900 mx-1 p-2 rounded-md flex-grow" type="text" />
                    </div>
                    <br>
                    <app-merged-sets-group-table [groups]="groups"></app-merged-sets-group-table>
                </div>
                <div class="flex flex-col w-full ml-1">
                    <label class="text-gray-400 italic">Add datasets</label>
                    <div class="flex">
                        <button *ngIf="groups.length === 0" disabled class="bg-gray-600 p-2 rounded-md">Add</button>
                        <button *ngIf="groups.length > 0" (click)="addSet(addedGroup, addedSets)"
                            class="bg-green-600 hover:bg-green-300 p-2 rounded-md">Add</button>
                        <select [disabled]="groups.length === 0" [(ngModel)]="addedGroup"
                            class="bg-gray-900 p-2 rounded-md w-3/12 mx-1">
                            <option *ngFor="let group of groups" [ngValue]="group">{{group}}</option>
                        </select>
                        <select [disabled]="groups.length === 0" [(ngModel)]="addedSets"
                            class="bg-gray-900 p-2 rounded-md flex-grow">
                            <option *ngFor="let raw of rawSets" [ngValue]="raw">{{raw.metadata.name}} v{{raw.metadata.version}}
                            </option>
                        </select>
                    </div>
                    <br>
                    <table class="w-full">
                        <tr>
                            <th class="bg-gray-500 p-2 rounded-l-md text-left">Name</th>
                            <th class="bg-gray-500 p-2 rounded-r-md">Group</th>
                        </tr>
                        <ng-container *ngIf="included.length > 0">
                            <ng-container *ngFor="let include of included">
                                <tr>
                                    <td class="p-0.5"></td>
                                </tr>
                                <tr>
                                    <td class="bg-gray-600 p-2 rounded-l-md text-left">{{include.names}}
                                        v{{include.version}}
                                    </td>
                                    <td class="bg-gray-600 p-2 rounded-r-md text-center">{{include.group}}</td>
                                </tr>
                            </ng-container>
                        </ng-container>
                        <ng-container *ngIf="included.length === 0">
                            <tr>
                                <td class="p-0.5"></td>
                            </tr>
                            <tr>
                                <td colspan="2" class="bg-gray-600 p-2 rounded-md text-center text-gray-300">None</td>
                            </tr>
                        </ng-container>
                    </table>
                </div>
            </div>
            <br>

            <h1 class="text-xl">Mapping</h1>
            <label class="text-gray-400 italic">Add Class</label>
            <div class="flex">
                <button (click)="addClasses(newClass)" class="bg-green-600 hover:bg-green-300 p-2 rounded-md">Add</button>
                <input [(ngModel)]="newClass" class="bg-gray-900 mx-1 p-2 rounded-md flex-grow" type="text" />
            </div>
            <br>
            <div class="flex flex-row w-full">
                <div class="flex flex-col w-full ml-1">
                    <table class="w-full">
                        <tr>
                            <th class="bg-gray-500 p-2 rounded-md">Final classes</th>
                        </tr>
                        <ng-container *ngIf="classes.length > 0">
                            <ng-container *ngFor="let news of classes">
                                <tr>
                                    <td class="p-0.5"></td>
                                </tr>
                                <tr>
                                    <td class="bg-gray-600 p-2 rounded-md text-center">{{news}}</td>
                                </tr>
                            </ng-container>
                        </ng-container>

                        <ng-container *ngIf="classes.length === 0">
                            <tr>
                                <td class="p-0.5"></td>
                            </tr>
                            <tr>
                                <td class="bg-gray-600 p-2 rounded-md text-center text-gray-300">None</td>
                            </tr>
                        </ng-container>
                    </table>
                </div>
                <div class="flex flex-col w-full ml-1">
                    <table class="w-full">
                        <tr>
                            <th class="bg-gray-500 p-2 rounded-l-md text-left w-1/2">Datasets</th>
                            <th class="bg-gray-500 p-2 rounded-r-md w-1/2">Map to</th>
                        </tr>
                        <ng-container *ngIf="mapping.length > 0">
                            <ng-container *ngFor="let mapp of mapping">
                                <tr>
                                    <td class="p-0.5"></td>
                                </tr>
                                <tr>
                                    <td class="bg-gray-600 p-2 rounded-l-md text-left">{{mapp.from}}</td>
                                    <td class="bg-gray-600 p-2 rounded-r-md text-center">
                                        <select (change)="changeMapTo(mapp.from, $event.target)" [value]="mapp.to"
                                            class="bg-gray-900 p-2 rounded-md w-full">
                                            <option [selected]="mapp.to === null" disabled [value]="null">None</option>
                                            <ng-container *ngFor="let mapto of classes">
                                                <option [value]="mapto" [selected]="mapp.to === mapto">{{mapto}}</option>
                                            </ng-container>
                                        </select>
                                    </td>
                                </tr>
                            </ng-container>
                        </ng-container>
                        <ng-container *ngIf="mapping.length === 0">
                            <tr>
                                <td class="p-0.5"></td>
                            </tr>
                            <tr>
                                <td colspan="2" class="bg-gray-600 p-2 rounded-md text-center text-gray-300">None</td>
                            </tr>
                        </ng-container>
                    </table>
                </div>
            </div>
            <br>
            <button (click)="store()" class="bg-green-600 hover:bg-green-300 p-2 rounded-md">Merge</button>
        </div>
    `
})
export class MergedSetsCreateComponent {
    baseSet: MergedSet | null = null;
    mergedSets: MergedSet[] = [];
    rawSets: DsmSets[] = [];
    included: IncludedSet[] = [];
    groups: string[] = [];
    classes: string[] = [];

    name = "";
    version = "";

    // Sub form
    inputGroup = "";
    addedGroup = "";
    addedSets: DsmSets | null = null;
    newClass = "";

    mapping: Mapping[] = []

    constructor() {
        listMergedDatasets().then((sets) => this.mergedSets = sets);
        listRawDatasets().then((sets) => this.rawSets = sets);
    }

    selectMerged(merged: MergedSet) {
        this.baseSet = merged;
    }

    addGroup(group: string) {
        if (!this.groups.includes(group))
            this.groups.push(group)
    }

    addSet(group: string, dataset: DsmSets | null) {
        if (dataset !== null) {
            const names = dataset.metadata.name;
            const version = dataset.metadata.version;
            let found = false;
            for (const included of this.included) {
                if (included.names === names && included.version === version) {
                    included.group = group;
                    found = true;
                    break
                }
            }
            if (!found) {
                this.included.push({
                    names,
                    version,
                    group
                })
                for (const dsmClasses of Object.values(dataset.metadata.classes)) {
                    const cla = dsmClasses.class;
                    let containClass = false;
                    for (const mapping of this.mapping) {
                        if (mapping.from === cla) {
                            containClass = true;
                            break;
                        }
                    }
                    if (!containClass) {
                        let mapTo = null;
                        for (const classs of this.classes) {
                            if (classs === cla) {
                                mapTo = classs;
                                break;
                            }
                        }
                        this.mapping.push({
                            from: cla,
                            to: mapTo
                        })
                    }
                }
            }
        }
    }

    changeMapTo(from: string, to: any | null) {
        if (to) {
            for (const mapping of this.mapping) {
                if (mapping.from === from) {
                    mapping.to = to.value
                }
            }
        }
    }

    createMapping() {
        const map: ClassMapper = {
            classes: {},
            mapping: {},
            custom: null
        };
        for (let index = 0; index < this.classes.length; index++) {
            map.classes[this.classes[index]] = index;
        }
        for (const mapping of this.mapping) {
            if (mapping.to === null || mapping.to === undefined || map.classes[mapping.to] === undefined) {
                return null;
            }
            if (map.mapping[mapping.to] === undefined) {
                map.mapping[mapping.to] = [];
            }
            map.mapping[mapping.to] = [...map.mapping[mapping.to], mapping.from];
        }

        return map;
    }

    addClasses(classs: string) {
        for (const c of this.classes) {
            if (classs === c) {
                return;
            }
        }
        this.classes.push(classs);
        for (const mapping of this.mapping) {
            if (mapping.to === null && mapping.from === classs) {
                mapping.to = classs;
            }
        }
    }

    store() {
        const mapping = this.createMapping();
        if (mapping === null)
            return;
        if (this.baseSet !== null) {
            console.log(this)
        } else {
            mergeNew(this.name, this.version, this.included, mapping)
        }
    }

    protected readonly Object = Object;
}

