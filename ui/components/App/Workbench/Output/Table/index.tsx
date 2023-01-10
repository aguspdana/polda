import { CalendarIcon } from "components/icons/CalendarIcon";
import { TextIcon } from "components/icons/TextIcon";
import { memo } from "react";
import { IDataFrame } from "store";
import styles from "./Table.module.css";

interface Props {
  data: IDataFrame
}

export const Table = memo(function Table({ data }: Props) {
  return (
    <div className={styles.container}>
      <table>
        <thead>
          <tr>
            {data.columns.map(column => (
              <th key={column.name} style={{ width: 240 }}>
                <div className={styles.cell}><Icon datatype={column.datatype}/>{column.name}</div>
              </th>
            ))}
          </tr>
        </thead>

        <tbody>
          {getRows(data).map((row, r) => (
            <tr key={r}>
              {row.map((cell, c) => (
                <td key={c}>{cell}</td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
});

function getRows(data: IDataFrame): any[][] {
  if (data.columns.length === 0) {
    return [];
  }
  const rows: any[][] = data.columns[0].values.map(_ => []);
  for (let c = 0; c < data.columns.length; c++) {
    const { datatype, values } = data.columns[c];
    if (datatype === "Boolean") {
      for (let r = 0; r < values.length; r++) {
        const value = values[r]
        const cell = value
        ? "true"
        : value === false
          ? "false"
          : "";
        rows[r].push(cell);
      }
    } else {
      for (let r = 0; r < values.length; r++) {
        rows[r].push(values[r]);
      }
    }
  }
  return rows;
}

function Icon({ datatype: dataType }: { datatype: string }) {
  switch (dataType) {
    case "Utf8":
      return <TextIcon/>;
    case "Date":
      return <CalendarIcon/>;
    default:
      return null;
  }
}