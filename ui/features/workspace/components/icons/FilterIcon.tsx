export default function FilterIcon({ color }: { color: string }) {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <g clipPath="url(#clip0_142_186)">
        <path d="M16 0H0V2L6 8V16L10 12V8L16 2V0Z" fill={color}/>
      </g>
      <defs>
        <clipPath id="clip0_142_186">
          <rect width="16" height="16"/>
        </clipPath>
      </defs>
    </svg>
  );
}
