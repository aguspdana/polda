export function CastIcon({ color }: { color: string }) {
  return (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
    <g clipPath="url(#clip0_258_72)">
      <rect y="9" width="7" height="7" fill={color}/>
      <circle cx="12.5" cy="12.5" r="3.5" fill={color}/>
      <path d="M0 6L3.5 0L7 6H0Z" fill={color}/>
      <path d="M9 3.5L12.5 0L16 3.5L12.5 7L9 3.5Z" fill={color}/>
    </g>
    <defs>
      <clipPath id="clip0_258_72">
        <rect width="16" height="16"/>
      </clipPath>
    </defs>
  </svg>

  );
}