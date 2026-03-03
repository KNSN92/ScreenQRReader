interface Props {
  className?: string;
  shapeClassName?: string;
}

export function Icon({ className, shapeClassName }: Props) {
  return (
    <svg
      viewBox="0 0 490 490"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      <g>
        <path
          className={shapeClassName}
          d="M0,0v210h210V0H0ZM175,175H35V35h140v140Z"
        />
        <rect className={shapeClassName} x="70" y="70" width="70" height="70" />
      </g>
      <g>
        <path
          className={shapeClassName}
          d="M280,0v210h210V0h-210ZM455,175h-140V35h140v140Z"
        />
        <rect
          className={shapeClassName}
          x="350"
          y="70"
          width="70"
          height="70"
        />
      </g>
      <g>
        <path
          className={shapeClassName}
          d="M0,280v210h210v-210H0ZM175,455H35v-140h140v140Z"
        />
        <rect
          className={shapeClassName}
          x="70"
          y="350"
          width="70"
          height="70"
        />
      </g>
      <polygon
        className={shapeClassName}
        points="472.5 340.75 393.75 340.75 393.75 262.5 341.25 262.5 341.25 340.75 262.5 340.75 262.5 393.25 341.25 393.25 341.25 472.5 393.75 472.5 393.75 393.25 472.5 393.25 472.5 340.75"
      />
    </svg>
  );
}
