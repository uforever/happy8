<script>
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  import {
    Chart,
    Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell,
  } from 'flowbite-svelte';

  let records = $state([]);
  
  let options = $derived({
    chart: {
      height: '600px',
      maxWidth: '100%',
      type: 'area',
      fontFamily: 'Inter, sans-serif',
      dropShadow: {
        enabled: false
      },
      toolbar: {
        show: false
      }
    },
    tooltip: {
      enabled: true,
      x: {
        show: false
      }
    },
    fill: {
      type: 'gradient',
      gradient: {
        opacityFrom: 0.55,
        opacityTo: 0,
        shade: '#1C64F2',
        gradientToColors: ['#1C64F2']
      }
    },
    dataLabels: {
      enabled: false
    },
    stroke: {
      width: 4
    },
    grid: {
      show: false,
      strokeDashArray: 4,
      padding: {
        left: 2,
        right: 2,
        top: 0
      }
    },
    series: [
      {
        name: '间隔期数',
        // data: [6500, 6418, 6456, 6526, 6356, 6456],
        data: records.map(record => record.step).reverse(),
        color: '#1A56DB'
      }
    ],
    xaxis: {
      // categories: ['01 February', '02 February', '03 February', '04 February', '05 February', '06 February', '07 February'],
      categories: records.map(record => record.issue).reverse(),
      labels: {
        show: true
      },
      axisBorder: {
        show: true
      },
      axisTicks: {
        show: true
      }
    },
    yaxis: {
      show: true,
    }
  });

  const hash = window.location.hash.slice(1);
  const { isAny, conditions } = JSON.parse(atob(hash));
  const connector = isAny ? " OR " : " AND ";
  const whereClause = conditions.map(condition => {
    const selected = condition.selected.map(num => `has${num < 10 ? '0' + num : num}`).join(" + ");
    return `${selected} >= ${condition.hit}`;
  }).join(connector);

  invoke("advance", { whereClause }).then((res) => {
    records = res.reverse();
  }).catch((err) => {
    alert(err);
    getCurrentWindow().close();
  });

</script>

<div class="grid gap-4 mb-4 md:grid-cols-8 px-4">
  <div class="col-span-3">
    <Table striped>
      <caption class="p-5 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
        符合筛选条件的共有 {records.length + 1} 期
        <!-- <p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">{isAny ? "任一成立" : "同时成立"}</p> -->
      </caption>
      <TableHead>
        <TableHeadCell class="text-base">期号</TableHeadCell>
        <TableHeadCell class="text-base">间隔</TableHeadCell>
      </TableHead>
      <TableBody>
        {#each records as item}
          <TableBodyRow>
            <TableBodyCell>{item.issue}</TableBodyCell>
            <TableBodyCell>{item.step}</TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  </div>
  <div class="col-span-5">
    <Chart {options} />
  </div>
</div>


