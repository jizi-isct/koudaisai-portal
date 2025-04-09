import type {NextConfig} from "next";
import {withNx} from "@nx/next";

const nextConfig: NextConfig = withNx({
    output: "export",
    images: {
        unoptimized: true
    },
    basePath: "/admin",
    trailingSlash: true
});

export default nextConfig;
