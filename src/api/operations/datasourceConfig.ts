import { emit, listen, UnlistenFn } from "@tauri-apps/api/event";

class DatasourceConfig {
  async datasourceCombUpdated(callback: () => void): Promise<UnlistenFn> {
    return await listen<void>("update-datasource-comb", callback);
  }
  async updateDatasourceComb() {
    await emit<void>("update-datasource-comb");
  }
}

const datasourceConfigOperate = new DatasourceConfig();

export default datasourceConfigOperate;
