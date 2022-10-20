export default function SelectIcon({ color }: { color: string }) {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <g clipPath="url(#clip0_142_128)">
        <rect y="9" width="7" height="7" fill={color}/>
        <rect x="9" y="9" width="7" height="7" fill={color}/>
        <path d="M1 3L3 5L7 1" stroke={color} strokeWidth="2"/>
        <path d="M10 1L15 6" stroke={color} strokeWidth="2"/>
        <path d="M10 6L15 1" stroke={color} strokeWidth="2"/>
      </g>
      <defs>
        <clipPath id="clip0_142_128">
          <rect width="16" height="16"/>
        </clipPath>
      </defs>
    </svg>
  );
}
