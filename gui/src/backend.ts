import {invoke} from "@tauri-apps/api/core";
import {DsmSets} from "./backend.types";

export async function listRawDatasets() {
    return invoke<DsmSets[]>("list_raw_datasets", { })
}