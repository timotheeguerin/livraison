declare module "postject" {
    export interface InjectOptions {
        sentinelFuse: string;
        machoSegmentName?: string;
        overwrite?: string;
    }
    export function inject(filename: string, resourceName: string, resourceData: Buffer, options: InjectOptions): Promise<void>;
}