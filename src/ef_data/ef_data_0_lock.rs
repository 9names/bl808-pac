#[doc = "Register `ef_data_0_lock` reader"]
pub struct R(crate::R<EF_DATA_0_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_DATA_0_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_DATA_0_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_DATA_0_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_data_0_lock` writer"]
pub struct W(crate::W<EF_DATA_0_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_DATA_0_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EF_DATA_0_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_DATA_0_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_sec_lifecycle` reader - "]
pub type EF_SEC_LIFECYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_sec_lifecycle` writer - "]
pub type EF_SEC_LIFECYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_DATA_0_LOCK_SPEC, u8, u8, 4, O>;
#[doc = "Field `wr_lock_rsvd_0` reader - "]
pub type WR_LOCK_RSVD_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wr_lock_rsvd_0` writer - "]
pub type WR_LOCK_RSVD_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_DATA_0_LOCK_SPEC, u16, u16, 10, O>;
#[doc = "Field `wr_lock_boot_mode` reader - "]
pub type WR_LOCK_BOOT_MODE_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_boot_mode` writer - "]
pub type WR_LOCK_BOOT_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_dbg_pwd` reader - "]
pub type WR_LOCK_DBG_PWD_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_dbg_pwd` writer - "]
pub type WR_LOCK_DBG_PWD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_wifi_mac` reader - "]
pub type WR_LOCK_WIFI_MAC_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_wifi_mac` writer - "]
pub type WR_LOCK_WIFI_MAC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_0` reader - "]
pub type WR_LOCK_KEY_SLOT_0_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_0` writer - "]
pub type WR_LOCK_KEY_SLOT_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_1` reader - "]
pub type WR_LOCK_KEY_SLOT_1_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_1` writer - "]
pub type WR_LOCK_KEY_SLOT_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_2` reader - "]
pub type WR_LOCK_KEY_SLOT_2_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_2` writer - "]
pub type WR_LOCK_KEY_SLOT_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_3` reader - "]
pub type WR_LOCK_KEY_SLOT_3_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_3` writer - "]
pub type WR_LOCK_KEY_SLOT_3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_sw_usage_0` reader - "]
pub type WR_LOCK_SW_USAGE_0_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_sw_usage_0` writer - "]
pub type WR_LOCK_SW_USAGE_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_sw_usage_1` reader - "]
pub type WR_LOCK_SW_USAGE_1_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_sw_usage_1` writer - "]
pub type WR_LOCK_SW_USAGE_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_sw_usage_2` reader - "]
pub type WR_LOCK_SW_USAGE_2_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_sw_usage_2` writer - "]
pub type WR_LOCK_SW_USAGE_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_sw_usage_3` reader - "]
pub type WR_LOCK_SW_USAGE_3_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_sw_usage_3` writer - "]
pub type WR_LOCK_SW_USAGE_3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_11` reader - "]
pub type WR_LOCK_KEY_SLOT_11_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_11` writer - "]
pub type WR_LOCK_KEY_SLOT_11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_dbg_pwd` reader - "]
pub type RD_LOCK_DBG_PWD_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_dbg_pwd` writer - "]
pub type RD_LOCK_DBG_PWD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_0` reader - "]
pub type RD_LOCK_KEY_SLOT_0_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_0` writer - "]
pub type RD_LOCK_KEY_SLOT_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_1` reader - "]
pub type RD_LOCK_KEY_SLOT_1_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_1` writer - "]
pub type RD_LOCK_KEY_SLOT_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_2` reader - "]
pub type RD_LOCK_KEY_SLOT_2_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_2` writer - "]
pub type RD_LOCK_KEY_SLOT_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_3` reader - "]
pub type RD_LOCK_KEY_SLOT_3_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_3` writer - "]
pub type RD_LOCK_KEY_SLOT_3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_11` reader - "]
pub type RD_LOCK_KEY_SLOT_11_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_11` writer - "]
pub type RD_LOCK_KEY_SLOT_11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ef_sec_lifecycle(&self) -> EF_SEC_LIFECYCLE_R {
        EF_SEC_LIFECYCLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:13"]
    #[inline(always)]
    pub fn wr_lock_rsvd_0(&self) -> WR_LOCK_RSVD_0_R {
        WR_LOCK_RSVD_0_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&self) -> WR_LOCK_BOOT_MODE_R {
        WR_LOCK_BOOT_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&self) -> WR_LOCK_DBG_PWD_R {
        WR_LOCK_DBG_PWD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&self) -> WR_LOCK_WIFI_MAC_R {
        WR_LOCK_WIFI_MAC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&self) -> WR_LOCK_KEY_SLOT_0_R {
        WR_LOCK_KEY_SLOT_0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&self) -> WR_LOCK_KEY_SLOT_1_R {
        WR_LOCK_KEY_SLOT_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&self) -> WR_LOCK_KEY_SLOT_2_R {
        WR_LOCK_KEY_SLOT_2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&self) -> WR_LOCK_KEY_SLOT_3_R {
        WR_LOCK_KEY_SLOT_3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&self) -> WR_LOCK_SW_USAGE_0_R {
        WR_LOCK_SW_USAGE_0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_1(&self) -> WR_LOCK_SW_USAGE_1_R {
        WR_LOCK_SW_USAGE_1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_2(&self) -> WR_LOCK_SW_USAGE_2_R {
        WR_LOCK_SW_USAGE_2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_3(&self) -> WR_LOCK_SW_USAGE_3_R {
        WR_LOCK_SW_USAGE_3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wr_lock_key_slot_11(&self) -> WR_LOCK_KEY_SLOT_11_R {
        WR_LOCK_KEY_SLOT_11_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&self) -> RD_LOCK_DBG_PWD_R {
        RD_LOCK_DBG_PWD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&self) -> RD_LOCK_KEY_SLOT_0_R {
        RD_LOCK_KEY_SLOT_0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&self) -> RD_LOCK_KEY_SLOT_1_R {
        RD_LOCK_KEY_SLOT_1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&self) -> RD_LOCK_KEY_SLOT_2_R {
        RD_LOCK_KEY_SLOT_2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&self) -> RD_LOCK_KEY_SLOT_3_R {
        RD_LOCK_KEY_SLOT_3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_lock_key_slot_11(&self) -> RD_LOCK_KEY_SLOT_11_R {
        RD_LOCK_KEY_SLOT_11_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn ef_sec_lifecycle(&mut self) -> EF_SEC_LIFECYCLE_W<0> {
        EF_SEC_LIFECYCLE_W::new(self)
    }
    #[doc = "Bits 4:13"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_rsvd_0(&mut self) -> WR_LOCK_RSVD_0_W<4> {
        WR_LOCK_RSVD_0_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_boot_mode(&mut self) -> WR_LOCK_BOOT_MODE_W<14> {
        WR_LOCK_BOOT_MODE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_dbg_pwd(&mut self) -> WR_LOCK_DBG_PWD_W<15> {
        WR_LOCK_DBG_PWD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_wifi_mac(&mut self) -> WR_LOCK_WIFI_MAC_W<16> {
        WR_LOCK_WIFI_MAC_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_0(&mut self) -> WR_LOCK_KEY_SLOT_0_W<17> {
        WR_LOCK_KEY_SLOT_0_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_1(&mut self) -> WR_LOCK_KEY_SLOT_1_W<18> {
        WR_LOCK_KEY_SLOT_1_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_2(&mut self) -> WR_LOCK_KEY_SLOT_2_W<19> {
        WR_LOCK_KEY_SLOT_2_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_3(&mut self) -> WR_LOCK_KEY_SLOT_3_W<20> {
        WR_LOCK_KEY_SLOT_3_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_sw_usage_0(&mut self) -> WR_LOCK_SW_USAGE_0_W<21> {
        WR_LOCK_SW_USAGE_0_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_sw_usage_1(&mut self) -> WR_LOCK_SW_USAGE_1_W<22> {
        WR_LOCK_SW_USAGE_1_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_sw_usage_2(&mut self) -> WR_LOCK_SW_USAGE_2_W<23> {
        WR_LOCK_SW_USAGE_2_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_sw_usage_3(&mut self) -> WR_LOCK_SW_USAGE_3_W<24> {
        WR_LOCK_SW_USAGE_3_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_11(&mut self) -> WR_LOCK_KEY_SLOT_11_W<25> {
        WR_LOCK_KEY_SLOT_11_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_dbg_pwd(&mut self) -> RD_LOCK_DBG_PWD_W<26> {
        RD_LOCK_DBG_PWD_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_0(&mut self) -> RD_LOCK_KEY_SLOT_0_W<27> {
        RD_LOCK_KEY_SLOT_0_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_1(&mut self) -> RD_LOCK_KEY_SLOT_1_W<28> {
        RD_LOCK_KEY_SLOT_1_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_2(&mut self) -> RD_LOCK_KEY_SLOT_2_W<29> {
        RD_LOCK_KEY_SLOT_2_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_3(&mut self) -> RD_LOCK_KEY_SLOT_3_W<30> {
        RD_LOCK_KEY_SLOT_3_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_11(&mut self) -> RD_LOCK_KEY_SLOT_11_W<31> {
        RD_LOCK_KEY_SLOT_11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_data_0_lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_data_0_lock](index.html) module"]
pub struct EF_DATA_0_LOCK_SPEC;
impl crate::RegisterSpec for EF_DATA_0_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_data_0_lock::R](R) reader structure"]
impl crate::Readable for EF_DATA_0_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_data_0_lock::W](W) writer structure"]
impl crate::Writable for EF_DATA_0_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_data_0_lock to value 0"]
impl crate::Resettable for EF_DATA_0_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
