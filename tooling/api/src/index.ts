declare global {
    interface Window {
        __FLURX__: {
            invoke: <Out>(id: string, args: any[]) => Promise<Out>,
        }
    }
}

export const invoke = <Out>(
    id: string,
    ...args: any[]
): Promise<Out> => {
    return window.__FLURX__.invoke(id, [...args]);
}

