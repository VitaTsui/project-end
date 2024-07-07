use rbatis::rbdc::db::ExecResult;

pub fn is_ok(res: rbatis::Result<ExecResult>, e: &str) -> Result<(), &str> {
    match res {
        Ok(res) => {
            if res.rows_affected >= 1 {
                Ok(())
            } else {
                Err(e)
            }
        }
        Err(_) => Err(e),
    }
}
