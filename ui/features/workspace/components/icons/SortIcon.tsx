export default function SortIcon({ color }: { color: string }) {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <g clipPath="url(#clip0_142_288)">
        <path d="M1 5L4 2M4 2L7 5M4 2V16" stroke={color} strokeWidth="2"/>
        <path d="M15 11L12 14M12 14L9 11M12 14L12 0" stroke={color} strokeWidth="2"/>
      </g>
      <defs>
        <clipPath id="clip0_142_288">
          <rect width="16" height="16"/>
        </clipPath>
      </defs>
    </svg>
  );
}
