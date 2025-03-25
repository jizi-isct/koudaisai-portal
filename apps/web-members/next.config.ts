import type {NextConfig} from "next";
import {withNx} from "@nx/next";

const nextConfig: NextConfig = withNx({
    output: "export"
});

export default nextConfig;
