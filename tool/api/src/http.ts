import {invoke} from "./core";

export interface RequestOptions{
    body?: BodyInit | null;
    headers?: HeadersInit;
    method?: string;
}

export namespace http{
    export const fetch = async (
        request: string | URL,
        options?: RequestOptions,
    ): Promise<Response> => {
        if(options?.body){
            // @ts-ignore
            options.body = Array.from(await new Response(options.body).bytes())
        }
        const output =  await invoke<{
            body: number[];
            headers: Record<string, string>;
            status: number;
            statusText: string;
        }>("FLURX|http::fetch", {
            url: request.toString(),
            ...options,
        });
        return new Response(new Uint8Array(output.body), {
            headers: output.headers,
            status: output.status,
            statusText: output.statusText,
        })
    }
}