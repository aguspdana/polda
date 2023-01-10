export function TrashBinIcon({ color } : { color: string }) {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="1" y="1" width="14" height="2" rx="1" fill={color}/>
      <path d="M5 1C5 0.447715 5.44772 0 6 0H10C10.5523 0 11 0.447715 11 1V1H5V1Z" fill={color}/>
      <path fill-rule="evenodd" clip-rule="evenodd" d="M3 5H13V14C13 15.1046 12.1046 16 11 16H5C3.89543 16 3 15.1046 3 14V5ZM5 8C5 7.44772 5.44772 7 6 7C6.55228 7 7 7.44772 7 8V13C7 13.5523 6.55228 14 6 14C5.44772 14 5 13.5523 5 13V8ZM10 7C9.44772 7 9 7.44772 9 8V13C9 13.5523 9.44772 14 10 14C10.5523 14 11 13.5523 11 13V8C11 7.44772 10.5523 7 10 7Z" fill={color}/>
    </svg>
  )
}
