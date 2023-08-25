import { emit, once } from "@tauri-apps/api/event";

class DatasourceConfig {
  async datasourceCombUpdated(callback: () => void) {
    await once<void>("update-datasource-comb", callback);
  }
  async updateDatasourceComb() {
    await emit<void>("update-datasource-comb");
  }
}

const datasourceConfigOperate = new DatasourceConfig();

export default datasourceConfigOperate;
