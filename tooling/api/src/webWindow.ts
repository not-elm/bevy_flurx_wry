import {invoke} from "./core";


export class WebWindow{
    private constructor(
        private readonly identifier: string,
    ) {
    }
    async title(): Promise<string>{
        return await invoke("FLURX|web_window::title", this.identifier)
    }

    static current(): WebWindow{
        return new WebWindow("<CURRENT_IDENTIFIER>");
    }
}
