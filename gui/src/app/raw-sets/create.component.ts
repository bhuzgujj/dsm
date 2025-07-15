import { Component } from '@angular/core';
import { storeRawSet } from "../../backend";
import { NgIf } from "@angular/common";
import { open } from '@tauri-apps/plugin-dialog';
import { FormsModule } from '@angular/forms';

@Component({
    selector: 'app-raw-sets-create',
    standalone: true,
    imports: [
        NgIf,
        FormsModule
    ],
    template: `
        <div class="w-full flex flex-col bg-gray-700 p-2 rounded-md">
            <h1 class="text-xl">Create Raw Sets</h1>
            <button class="bg-gray-600 hover:bg-gray-300 p-2 rounded-md" (click)="selectOnDisk()">Reselect path...</button>
            <br>
            <label class="text-gray-400 italic">Name</label>
            <input *ngIf="path === null" [(ngModel)]="name" [disabled]="path === null"
                class="bg-gray-800 p-2 rounded-md" type="text" />
            <input *ngIf="path !== null" [(ngModel)]="name" [disabled]="path === null"
                class="bg-gray-900 p-2 rounded-md" type="text" />
            <br>
            <label class="text-gray-400 italic">Version</label>
            <input *ngIf="path === null" [(ngModel)]="version" [disabled]="path === null"
                class="bg-gray-800 p-2 rounded-md" type="text" />
            <input *ngIf="path !== null" [(ngModel)]="version" [disabled]="path === null"
                class="bg-gray-900 p-2 rounded-md" type="text" />
            <br>
            <button *ngIf="path === null || name.length === 0 || version.length === 0" (click)="store()"
                [disabled]="path === null" class="bg-gray-600 p-2 rounded-md text-gray-500">Store</button>
            <button *ngIf="path !== null && name.length > 0 && version.length > 0" (click)="store()"
                [disabled]="path === null" class="bg-gray-600 hover:bg-gray-300 p-2 rounded-md">Store</button>
        </div>
    `
})
export class RawSetsCreateComponent {
    path: string | null = null;
    name = "";
    version = "";
    formats: string = "yolo-1-1";

    constructor() {
        this.selectOnDisk();
    }

    selectOnDisk() {
        open({
            multiple: false,
            directory: true
        }).then((file) => {
            if (file) {
                this.path = file;
                this.name = file.split('\\').at(-1) || "";
            }
        });
    }

    store() {
        if (this.path !== null) {
            storeRawSet(this.name, this.version, this.formats, this.path)
                .then(() => window.location.href = "/raw-sets")
                .catch(console.error);
        }
    }

    protected readonly Object = Object;
}
