<script>
  import { invoke } from "@tauri-apps/api/core";
  import {
    Button, FloatingLabelInput, MultiSelect, Modal, Label
  } from "flowbite-svelte";
  import { CheckCircleOutline, ExclamationCircleOutline } from 'flowbite-svelte-icons';

  let alertContent = $state("");
  let isWarning = $state(true);
  let popupModal = $state(false);

  let issue = $state("");
  let selected = $state([]);

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
    if (!issue) {
      myAlert("请输入期号", true);
      return;
    }
    if (selected.length !== 20) {
      myAlert("中奖号码数量不等于20", true);
      return;
    }
    const item = { issue, winning: selected };
    invoke("add", { item }).then((res) => {
      myAlert(res, res !== "录入成功");
      issue = "";
      selected = [];
    }).catch((e) => {
      myAlert(e, true);
    });
  }
</script>

<form {onsubmit}>
  <div class="mb-6">
    <FloatingLabelInput style="outlined" bind:value={issue} type="text">
      期号
    </FloatingLabelInput>
  </div>
  <div class="mb-6">
    <Label class="mb-2">中奖号码</Label>
    <MultiSelect items={numbers} bind:value={selected} placeholder='中奖号码' />
  </div>
  <div class="flex justify-center">
    <Button type="submit">添加</Button>
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
    <!-- <Button color="alternative">No, cancel</Button> -->
  </div>
</Modal>