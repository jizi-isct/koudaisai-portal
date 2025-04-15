import {withNx} from "@nx/next";

const nextConfig = withNx({
    output: "export",
    images: {
        unoptimized: true
    },
    basePath: "/admin",
    trailingSlash: true,
});

export default nextConfig;
