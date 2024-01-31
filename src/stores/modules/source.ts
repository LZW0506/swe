import {defineStore} from "pinia";

interface stateType {
    selectKey: string | number | null
}

export const useSourceStore = defineStore(
    'dict',
    {
        state: (): stateType => ({
            selectKey: null,
        }),
        actions: {
            setSelectKey(key: string | number | null) {
                this.selectKey = key
            }
        }
    }
)