import { useRef } from "react";
import clsx from "clsx";

export function Loading() {
  return (
    <div className="p-16">
      <LoadingSvg />
    </div>
  );
}

export function LoadingSvg() {
  const className = "fill-stone-400";

  const svgRef = useRef<SVGSVGElement>(null);

  return (
    <svg
      ref={svgRef}
      viewBox="0 0 490 490"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <g className="animate-[loading-lu_3s_ease-in-out_infinite]">
        <path
          className={className}
          d="M0,0v210h210V0H0ZM175,175H35V35h140v140Z"
        />
        <rect className={className} x="70" y="70" width="70" height="70" />
      </g>
      <g className="animate-[loading-ru_3s_ease-in-out_infinite]">
        <path
          className={className}
          d="M280,0v210h210V0h-210ZM455,175h-140V35h140v140Z"
        />
        <rect className={className} x="350" y="70" width="70" height="70" />
      </g>
      <g className="animate-[loading-ld_3s_ease-in-out_infinite]">
        <path
          className={className}
          d="M0,280v210h210v-210H0ZM175,455H35v-140h140v140Z"
        />
        <rect className={className} x="70" y="350" width="70" height="70" />
      </g>
      <polygon
        className={clsx(
          className,
          "animate-[loading-rd_3s_ease-in-out_infinite]",
        )}
        points="472.5 340.75 393.75 340.75 393.75 262.5 341.25 262.5 341.25 340.75 262.5 340.75 262.5 393.25 341.25 393.25 341.25 472.5 393.75 472.5 393.75 393.25 472.5 393.25 472.5 340.75"
      />
    </svg>
  );
}
