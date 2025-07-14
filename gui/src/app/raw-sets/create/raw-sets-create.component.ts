import { Component } from '@angular/core';
import { storeRawSet } from "../../../backend";
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
    templateUrl: './raw-sets-create.component.html'
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
