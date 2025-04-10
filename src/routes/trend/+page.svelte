<script>
  // import { invoke } from "@tauri-apps/api/core";
  import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
  import {
    Button, FloatingLabelInput, MultiSelect, Modal, CloseButton,
    Hr, Toggle
  } from "flowbite-svelte";
  import { PlusOutline, CheckCircleOutline, ExclamationCircleOutline } from 'flowbite-svelte-icons';

  let alertContent = $state("");
  let isWarning = $state(true);
  let popupModal = $state(false);

  let isAny = $state(false);

  let conditions = $state([
    { selected: [], hit: 0 },
  ]);
  
  const numbers = Array.from({ length: 80 }, (_, index) => ({
    value: index + 1,
    name: (index + 1).toString()
  }));

  function myAlert(content, warning) {
    alertContent = content;
    isWarning = warning;
    popupModal = true;
  }

  function onsubmit(event) {
    event.preventDefault();

    // myAlert(JSON.stringify({ isAny, conditions }), true);
    new WebviewWindow('advance-window', {
      url: `/chart#${btoa(JSON.stringify({ isAny, conditions }))}`,
    });
  }

  function handleAddCondition() {
    conditions = [...conditions, { selected: [], hit: 0 }];
  }
</script>


<form {onsubmit}>
  <Hr classHr="my-8 w-64" icon>
    <Button outline size="xs" onclick={handleAddCondition}>
      <PlusOutline class="w-3 h-3 me-1" /> 增加筛选条件
    </Button>  
  </Hr>

  {#each conditions as condition, index (index)}
  <div class="grid gap-3 mb-3 md:grid-cols-12">
    <div class="col-span-9">
      <MultiSelect items={numbers} bind:value={condition.selected} placeholder='号码组' size="sm" />
    </div>
    <div class="col-span-2">
      <FloatingLabelInput style="outlined" type="number" size="sm" min=0 bind:value={condition.hit} max={condition.selected.length}>
        命中数
      </FloatingLabelInput>
    </div>
    <div class="col-span-1">
      <CloseButton onclick={() => {
        conditions = conditions.filter((_, i) => i !== index);
      }} />
    </div>
  </div>
  {/each}

  <div class="flex justify-center mb-4">
    <Toggle size="small" class="mt-3" bind:checked={isAny}>条件{isAny ? "任一成立" : "同时成立"}</Toggle>
  </div>

  <div class="flex justify-center">
    <Button type="submit">查询</Button>
  </div>
</form>

<Modal bind:open={popupModal} size="xs" autoclose>
  <div class="text-center">
    {#if isWarning}
      <ExclamationCircleOutline class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200" />
    {:else}
      <CheckCircleOutline class="mx-auto mb-4 text-green-600 w-12 h-12" />
    {/if}
    
    <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">{alertContent}</h3>
    <Button color="red" class="me-2">确定</Button>
  </div>
</Modal>